#include "GltfLoader.h"
#define CGLTF_IMPLEMENTATION
#include <cgltf/cgltf.h>
#include <iostream>
#include <cstring>
#include <cmath>
#include <algorithm>
#include <cctype>
#include <charconv>

GltfLoader::GltfLoader() {}
GltfLoader::~GltfLoader() {}

namespace
{
    struct CgltfScopeGuard
    {
        cgltf_data* data = nullptr;
        ~CgltfScopeGuard() { if (data) cgltf_free(data); }
    };

    void DecomposeMatrixColMajor(const float m[16], float3& outTranslation, float4& outRotation, float3& outScale)
    {
        // Translacja (kolumna 3 w formacie kolumnowym glTF)
        outTranslation = { m[12], m[13], m[14] };

        // Skala (dlugosci kolumn bazy 0, 1, 2)
        float sx = std::sqrt(m[0] * m[0] + m[1] * m[1] + m[2] * m[2]);
        float sy = std::sqrt(m[4] * m[4] + m[5] * m[5] + m[6] * m[6]);
        float sz = std::sqrt(m[8] * m[8] + m[9] * m[9] + m[10] * m[10]);

        if (sx < 1e-6f) sx = 1.0f;
        if (sy < 1e-6f) sy = 1.0f;
        if (sz < 1e-6f) sz = 1.0f;

        // Normalizacja kolumn rotacji
        float r00 = m[0] / sx, r10 = m[1] / sx, r20 = m[2] / sx;
        float r01 = m[4] / sy, r11 = m[5] / sy, r21 = m[6] / sy;
        float r02 = m[8] / sz, r12 = m[9] / sz, r22 = m[10] / sz;

        // Sprawdzenie wyznacznika pod katem odbicia lustrzanego (ujemna skala)
        float det = r00 * (r11 * r22 - r12 * r21) - r01 * (r10 * r22 - r12 * r20) + r02 * (r10 * r21 - r11 * r20);
        if (det < 0.0f)
        {
            sz = -sz;
            r02 = -r02;
            r12 = -r12;
            r22 = -r22;
        }

        outScale = { sx, sy, sz };

        // Rotacja z macierzy ortonormalnej 3x3 do kwaternionu (x, y, z, w)
        float trace = r00 + r11 + r22;
        float qx = 0.0f, qy = 0.0f, qz = 0.0f, qw = 1.0f;

        if (trace > 0.0f)
        {
            float s = 0.5f / std::sqrt(trace + 1.0f);
            qw = 0.25f / s;
            qx = (r21 - r12) * s;
            qy = (r02 - r20) * s;
            qz = (r10 - r01) * s;
        }
        else
        {
            if (r00 > r11 && r00 > r22)
            {
                float s = 2.0f * std::sqrt(1.0f + r00 - r11 - r22);
                qw = (r21 - r12) / s;
                qx = 0.25f * s;
                qy = (r01 + r10) / s;
                qz = (r02 + r20) / s;
            }
            else if (r11 > r22)
            {
                float s = 2.0f * std::sqrt(1.0f + r11 - r00 - r22);
                qw = (r02 - r20) / s;
                qx = (r01 + r10) / s;
                qy = 0.25f * s;
                qz = (r12 + r21) / s;
            }
            else
            {
                float s = 2.0f * std::sqrt(1.0f + r22 - r00 - r11);
                qw = (r10 - r01) / s;
                qx = (r02 + r20) / s;
                qy = (r12 + r21) / s;
                qz = 0.25f * s;
            }
        }

        float qlen = std::sqrt(qx * qx + qy * qy + qz * qz + qw * qw);
        if (qlen > 1e-6f)
        {
            qx /= qlen; qy /= qlen; qz /= qlen; qw /= qlen;
        }
        else
        {
            qx = 0.0f; qy = 0.0f; qz = 0.0f; qw = 1.0f;
        }

        outRotation = { qx, qy, qz, qw };
    }
}

bool GltfLoader::LoadFromFile(const std::string& path, GltfModelData& outModelData)
{
    cgltf_options options = {};
    cgltf_data* data = nullptr;
    if (cgltf_parse_file(&options, path.c_str(), &data) != cgltf_result_success) return false;
    CgltfScopeGuard guard{data};
    if (cgltf_load_buffers(&options, data, path.c_str()) != cgltf_result_success)
    {
        return false;
    }
    return ProcessData(data, outModelData);
}

bool GltfLoader::LoadFromMemory(const void* bufferData, size_t size, GltfModelData& outModelData)
{
    cgltf_options options = {};
    cgltf_data* data = nullptr;
    if (cgltf_parse(&options, bufferData, size, &data) != cgltf_result_success) return false;
    CgltfScopeGuard guard{data};
    if (cgltf_load_buffers(&options, data, nullptr) != cgltf_result_success)
    {
        return false;
    }
    return ProcessData(data, outModelData);
}

static std::string GetExtrasJsonString(const cgltf_data* data, const cgltf_extras& extras)
{
    if (extras.data && strlen(extras.data) > 0)
    {
        return std::string(extras.data);
    }
    if (extras.end_offset > extras.start_offset && data && data->json)
    {
        cgltf_size size = 0;
        if (cgltf_copy_extras_json(data, &extras, nullptr, &size) == cgltf_result_success && size > 1)
        {
            std::string buf(size, '\0');
            if (cgltf_copy_extras_json(data, &extras, &buf[0], &size) == cgltf_result_success)
            {
                if (!buf.empty() && buf.back() == '\0') buf.pop_back();
                return buf;
            }
        }
    }
    return "";
}

static void ParseMetin2Events(const std::string& json, std::vector<GltfEvent>& outEvents)
{
    size_t pos = json.find("\"metin2_events\"");
    if (pos == std::string::npos) return;

    size_t arrStart = json.find('[', pos);
    size_t arrEnd = json.find(']', arrStart);
    if (arrStart == std::string::npos || arrEnd == std::string::npos) return;

    auto extractStringField = [](const std::string& obj, const char* key) -> std::string {
        size_t keyPos = obj.find(key);
        if (keyPos == std::string::npos) return "";
        size_t colon = obj.find(':', keyPos);
        if (colon == std::string::npos) return "";
        size_t q1 = obj.find('\"', colon + 1);
        if (q1 == std::string::npos) return "";
        size_t q2 = obj.find('\"', q1 + 1);
        if (q2 == std::string::npos) return "";
        return obj.substr(q1 + 1, q2 - q1 - 1);
    };

    size_t cur = arrStart + 1;
    while (cur < arrEnd)
    {
        size_t objStart = json.find('{', cur);
        if (objStart == std::string::npos || objStart >= arrEnd) break;
        size_t objEnd = json.find('}', objStart);
        if (objEnd == std::string::npos || objEnd > arrEnd) break;

        std::string objStr = json.substr(objStart, objEnd - objStart + 1);
        GltfEvent ev{};
        ev.time = 0.0f;

        size_t timePos = objStr.find("\"time\"");
        if (timePos != std::string::npos)
        {
            size_t colon = objStr.find(':', timePos);
            if (colon != std::string::npos)
            {
                size_t numStart = objStr.find_first_not_of(" \t\r\n", colon + 1);
                if (numStart != std::string::npos)
                {
                    std::from_chars(objStr.data() + numStart, objStr.data() + objStr.size(), ev.time);
                }
            }
        }

        ev.type = extractStringField(objStr, "\"type\"");
        ev.arg  = extractStringField(objStr, "\"arg\"");

        if (!ev.type.empty())
        {
            outEvents.push_back(std::move(ev));
        }

        cur = objEnd + 1;
    }
}

static void ParseMaterialExtras(const std::string& json, std::string& outDiffuse, std::string& outOpacity)
{
    auto extractString = [&](const std::string& key) -> std::string {
        size_t keyPos = json.find("\"" + key + "\"");
        if (keyPos == std::string::npos) return "";
        size_t colon = json.find(':', keyPos);
        if (colon == std::string::npos) return "";
        size_t q1 = json.find('\"', colon + 1);
        if (q1 == std::string::npos) return "";
        size_t q2 = json.find('\"', q1 + 1);
        if (q2 == std::string::npos) return "";
        return json.substr(q1 + 1, q2 - q1 - 1);
    };

    outDiffuse = extractString("diffuse_texture");
    outOpacity = extractString("opacity_texture");
}

void GltfLoader::ExtractEvents(cgltf_data* data, GltfModelData& outModelData)
{
    if (!data) return;

    for (size_t i = 0; i < data->animations_count && i < outModelData.motions.size(); ++i)
    {
        const cgltf_animation& anim = data->animations[i];
        std::string animExtras = GetExtrasJsonString(data, anim.extras);
        if (!animExtras.empty())
        {
            ParseMetin2Events(animExtras, outModelData.motions[i].events);
        }
    }

    std::string assetExtras = GetExtrasJsonString(data, data->asset.extras);
    if (!assetExtras.empty())
    {
        std::vector<GltfEvent> globalEvents;
        ParseMetin2Events(assetExtras, globalEvents);
        if (!globalEvents.empty())
        {
            for (auto& motion : outModelData.motions)
            {
                if (motion.events.empty())
                {
                    motion.events = globalEvents;
                }
            }
        }
    }
}

bool GltfLoader::ProcessData(cgltf_data* data, GltfModelData& outModelData)
{
    if (!data) return false;

    // Materialy
    outModelData.materials.reserve(data->materials_count);
    for (size_t i = 0; i < data->materials_count; ++i)
    {
        const cgltf_material& mat = data->materials[i];
        GltfMaterial m;
        m.name = mat.name ? mat.name : "Material_" + std::to_string(i);
        m.doubleSided = mat.double_sided ? true : false;

        std::string matExtras = GetExtrasJsonString(data, mat.extras);
        if (!matExtras.empty())
        {
            ParseMaterialExtras(matExtras, m.diffuseTexture, m.opacityTexture);
        }

        if (m.diffuseTexture.empty() &&
            mat.has_pbr_metallic_roughness &&
            mat.pbr_metallic_roughness.base_color_texture.texture &&
            mat.pbr_metallic_roughness.base_color_texture.texture->image &&
            mat.pbr_metallic_roughness.base_color_texture.texture->image->uri)
        {
            m.diffuseTexture = mat.pbr_metallic_roughness.base_color_texture.texture->image->uri;
        }

        outModelData.materials.push_back(m);
    }

    // Meshes (Geometria)
    for (size_t i = 0; i < data->meshes_count; ++i)
    {
        const cgltf_mesh& mesh = data->meshes[i];
        for (size_t j = 0; j < mesh.primitives_count; ++j)
        {
            const cgltf_primitive& prim = mesh.primitives[j];
            if (prim.type != cgltf_primitive_type_triangles)
                continue;
            
            GltfSubmesh submesh;
            submesh.name = mesh.name ? mesh.name : "unnamed_mesh";
            submesh.materialIndex = prim.material ? (int)cgltf_material_index(data, prim.material) : -1;
            submesh.vertexOffset = (unsigned int)outModelData.vertices.size();
            submesh.indexOffset = (unsigned int)outModelData.indices.size();

            size_t vertexCount = 0;
            for (size_t k = 0; k < prim.attributes_count; ++k)
            {
                if (prim.attributes[k].data->count > vertexCount)
                    vertexCount = prim.attributes[k].data->count;
            }
            submesh.vertexCount = (unsigned int)vertexCount;

            size_t startVertex = outModelData.vertices.size();
            outModelData.vertices.resize(startVertex + vertexCount);

            // Wyciaganie atrybutow z bounds-checking
            for (size_t k = 0; k < prim.attributes_count; ++k)
            {
                const cgltf_attribute& attr = prim.attributes[k];
                for (size_t v = 0; v < attr.data->count; ++v)
                {
                    GltfVertex& vertex = outModelData.vertices[startVertex + v];
                    float values[4] = {0};
                    cgltf_accessor_read_float(attr.data, v, values, 4);

                    if (attr.type == cgltf_attribute_type_position) {
                        vertex.position = {values[0], values[1], values[2]};
                    } else if (attr.type == cgltf_attribute_type_normal) {
                        vertex.normal = {values[0], values[1], values[2]};
                    } else if (attr.type == cgltf_attribute_type_texcoord) {
                        if (attr.index == 0) vertex.uv0 = {values[0], values[1]};
                        else if (attr.index == 1) vertex.uv1 = {values[0], values[1]};
                    } else if (attr.type == cgltf_attribute_type_joints) {
                        cgltf_uint joints[4] = {0, 0, 0, 0};
                        cgltf_accessor_read_uint(attr.data, v, joints, 4);
                        vertex.jointIndices = {joints[0], joints[1], joints[2], joints[3]};
                    } else if (attr.type == cgltf_attribute_type_weights) {
                        vertex.jointWeights = {values[0], values[1], values[2], values[3]};
                    }
                }
            }

            // Indeksy (Index buffery z poprawnym offsetem wierzcholkow submesha)
            if (prim.indices)
            {
                submesh.indexCount = (unsigned int)prim.indices->count;
                size_t startIndex = outModelData.indices.size();
                outModelData.indices.resize(startIndex + submesh.indexCount);
                for (size_t idx = 0; idx < prim.indices->count; ++idx)
                {
                    outModelData.indices[startIndex + idx] = (unsigned int)cgltf_accessor_read_index(prim.indices, idx) + (unsigned int)startVertex;
                }
            }
            else
            {
                submesh.indexCount = (unsigned int)vertexCount;
                size_t startIndex = outModelData.indices.size();
                outModelData.indices.resize(startIndex + submesh.indexCount);
                for (size_t idx = 0; idx < vertexCount; ++idx)
                {
                    outModelData.indices[startIndex + idx] = (unsigned int)(startVertex + idx);
                }
            }

            outModelData.submeshes.push_back(submesh);
        }
    }

    // Naprawa indeksow materialow dla plikow .glb wyeksportowanych z lokalnym MaterialIndex (wskazujacym na puste Map #0)
    std::vector<int> validMatIndices;
    for (size_t i = 0; i < outModelData.materials.size(); ++i)
    {
        if (!outModelData.materials[i].diffuseTexture.empty())
        {
            validMatIndices.push_back((int)i);
        }
    }

    if (!validMatIndices.empty())
    {
        auto toLowerStr = [](std::string s) {
            std::transform(s.begin(), s.end(), s.begin(), [](unsigned char c) { return (char)std::tolower(c); });
            return s;
        };

        auto isDetailMat = [&](int idx) -> bool {
            if (idx < 0 || idx >= (int)outModelData.materials.size()) return false;
            std::string texLow = toLowerStr(outModelData.materials[idx].diffuseTexture);
            std::string nameLow = toLowerStr(outModelData.materials[idx].name);
            const char* detailKeywords[] = {
                "teeth", "tooth", "face", "eye", "hair", "tongue",
                "horn", "wing", "feather", "tail", "weapon", "glass"
            };
            for (const char* kw : detailKeywords)
            {
                if (texLow.find(kw) != std::string::npos || nameLow.find(kw) != std::string::npos)
                    return true;
            }
            return false;
        };

        // Znajdz submeshe, ktore maja juz poprawny material, i oznacz te materialy jako zajete (claimed)
        std::vector<bool> matClaimed(outModelData.materials.size(), false);
        size_t maxVertexCount = 0;
        for (size_t s = 0; s < outModelData.submeshes.size(); ++s)
        {
            if (outModelData.submeshes[s].vertexCount > maxVertexCount)
                maxVertexCount = outModelData.submeshes[s].vertexCount;

            int midx = outModelData.submeshes[s].materialIndex;
            if (midx >= 0 && midx < (int)outModelData.materials.size() && !outModelData.materials[midx].diffuseTexture.empty())
            {
                matClaimed[midx] = true;
            }
        }

        // Dopasowanie dla submeshy bez poprawnego materialu
        for (size_t s = 0; s < outModelData.submeshes.size(); ++s)
        {
            GltfSubmesh& sub = outModelData.submeshes[s];
            int midx = sub.materialIndex;
            if (midx >= 0 && midx < (int)outModelData.materials.size() && !outModelData.materials[midx].diffuseTexture.empty())
                continue; // Material ma juz poprawna teksture

            std::string subNameLow = toLowerStr(sub.name);
            bool isMainSubmesh = (maxVertexCount > 0 && sub.vertexCount >= (maxVertexCount * 2) / 3);
            bool isSubmeshDetail = (!isMainSubmesh && maxVertexCount > 0 && sub.vertexCount <= maxVertexCount / 3);

            // 1. Sprawdz czy submesh pasuje bezposrednio nazwa do slowa kluczowego w teksturze (np. "face", "teeth")
            int bestMatchIdx = -1;
            for (int idx : validMatIndices)
            {
                std::string texLow = toLowerStr(outModelData.materials[idx].diffuseTexture);
                std::string nameLow = toLowerStr(outModelData.materials[idx].name);
                if (subNameLow.find("face") != std::string::npos && (texLow.find("face") != std::string::npos || nameLow.find("face") != std::string::npos))
                {
                    bestMatchIdx = idx;
                    break;
                }
                if (subNameLow.find("teeth") != std::string::npos && (texLow.find("teeth") != std::string::npos || nameLow.find("teeth") != std::string::npos))
                {
                    bestMatchIdx = idx;
                    break;
                }
            }

            // 2. Jesli to glowny submesh (cialo), szukamy NIEZAJETEGO materialu BAZOWEGO (nie bedacego detalem typu teeth/face)
            if (bestMatchIdx < 0 && isMainSubmesh)
            {
                // Preferuj niezajety material nie-detaliczny
                for (int idx : validMatIndices)
                {
                    if (!matClaimed[idx] && !isDetailMat(idx))
                    {
                        bestMatchIdx = idx;
                        break;
                    }
                }
                // Jesli wszystkie sa zajete, preferuj dowolny nie-detaliczny
                if (bestMatchIdx < 0)
                {
                    for (int idx : validMatIndices)
                    {
                        if (!isDetailMat(idx))
                        {
                            bestMatchIdx = idx;
                            break;
                        }
                    }
                }
            }

            // 3. Jesli to detaliczny submesh, preferuj material detaliczny
            if (bestMatchIdx < 0 && isSubmeshDetail)
            {
                for (int idx : validMatIndices)
                {
                    if (!matClaimed[idx] && isDetailMat(idx))
                    {
                        bestMatchIdx = idx;
                        break;
                    }
                }
            }

            // 4. Szukaj dowolnego niezajetego poprawnego materialu
            if (bestMatchIdx < 0)
            {
                for (int idx : validMatIndices)
                {
                    if (!matClaimed[idx])
                    {
                        bestMatchIdx = idx;
                        break;
                    }
                }
            }

            // 5. Fallback: jesli midx miesci sie w validMatIndices
            if (bestMatchIdx < 0)
            {
                if (midx >= 0 && midx < (int)validMatIndices.size())
                    bestMatchIdx = validMatIndices[midx];
                else
                    bestMatchIdx = validMatIndices[0];
            }

            sub.materialIndex = bestMatchIdx;
            matClaimed[bestMatchIdx] = true;
        }
    }

    // Szkielet (Skin / Joints / InverseBindMatrices / Rest Pose)
    if (data->skins_count > 0)
    {
        const cgltf_skin& skin = data->skins[0];
        outModelData.skin.name = skin.name ? skin.name : "unnamed_skin";
        
        std::vector<float> invBindMatrices(skin.joints_count * 16, 0.0f);
        if (skin.inverse_bind_matrices) {
            for (size_t i = 0; i < skin.joints_count; ++i) {
                cgltf_accessor_read_float(skin.inverse_bind_matrices, i, &invBindMatrices[i * 16], 16);
            }
        } else {
            for (size_t i = 0; i < skin.joints_count; ++i) {
                invBindMatrices[i * 16 + 0] = 1.0f;
                invBindMatrices[i * 16 + 5] = 1.0f;
                invBindMatrices[i * 16 + 10] = 1.0f;
                invBindMatrices[i * 16 + 15] = 1.0f;
            }
        }

        for (size_t i = 0; i < skin.joints_count; ++i)
        {
            const cgltf_node* jointNode = skin.joints[i];
            GltfJoint joint;
            joint.name = jointNode->name ? jointNode->name : "unnamed_joint";
            joint.parentIndex = -1;
            
            if (jointNode->parent) {
                for (size_t p = 0; p < skin.joints_count; ++p) {
                    if (skin.joints[p] == jointNode->parent) {
                        joint.parentIndex = (int)p;
                        break;
                    }
                }
            }

            // Odczyt bind pose (rest pose) kosci
            if (jointNode->has_matrix) {
                DecomposeMatrixColMajor(jointNode->matrix, joint.localTranslation, joint.localRotation, joint.localScale);
            } else {
                if (jointNode->has_translation) {
                    joint.localTranslation = {jointNode->translation[0], jointNode->translation[1], jointNode->translation[2]};
                }
                if (jointNode->has_rotation) {
                    joint.localRotation = {jointNode->rotation[0], jointNode->rotation[1], jointNode->rotation[2], jointNode->rotation[3]};
                }
                if (jointNode->has_scale) {
                    joint.localScale = {jointNode->scale[0], jointNode->scale[1], jointNode->scale[2]};
                }
            }
            
            for (int r = 0; r < 4; ++r) {
                for (int c = 0; c < 4; ++c) {
                    joint.inverseBindMatrix.m[r][c] = invBindMatrices[i * 16 + r * 4 + c];
                }
            }
            outModelData.skin.joints.push_back(joint);
        }
    }

    // Animacje
    for (size_t i = 0; i < data->animations_count; ++i)
    {
        const cgltf_animation& anim = data->animations[i];
        GltfMotionData motion;
        motion.name = anim.name ? anim.name : "unnamed_animation";
        motion.duration = 0.0f;

        for (size_t c = 0; c < anim.channels_count; ++c)
        {
            const cgltf_animation_channel& channel = anim.channels[c];
            GltfAnimationChannel animChannel;
            
            animChannel.jointIndex = -1;
            if (channel.target_node && channel.target_node->name) {
                animChannel.targetNodeName = channel.target_node->name;
            }

            if (data->skins_count > 0) {
                const cgltf_skin& skin = data->skins[0];
                for (size_t j = 0; j < skin.joints_count; ++j) {
                    if (skin.joints[j] == channel.target_node) {
                        animChannel.jointIndex = (int)j;
                        break;
                    }
                }
            }
            
            if (channel.target_path == cgltf_animation_path_type_translation) animChannel.pathType = GltfAnimationPathType::Translation;
            else if (channel.target_path == cgltf_animation_path_type_rotation) animChannel.pathType = GltfAnimationPathType::Rotation;
            else if (channel.target_path == cgltf_animation_path_type_scale) animChannel.pathType = GltfAnimationPathType::Scale;
            else animChannel.pathType = GltfAnimationPathType::Unknown;
            
            const cgltf_animation_sampler* sampler = channel.sampler;
            if (sampler && sampler->input && sampler->output)
            {
                if (sampler->interpolation == cgltf_interpolation_type_step) {
                    animChannel.interpolation = GltfInterpolationType::Step;
                } else if (sampler->interpolation == cgltf_interpolation_type_cubic_spline) {
                    animChannel.interpolation = GltfInterpolationType::CubicSpline;
                } else {
                    animChannel.interpolation = GltfInterpolationType::Linear;
                }

                size_t inputCount = sampler->input->count;
                animChannel.times.resize(inputCount);
                for (size_t t = 0; t < inputCount; ++t) {
                    float time = 0;
                    cgltf_accessor_read_float(sampler->input, t, &time, 1);
                    animChannel.times[t] = time;
                    if (time > motion.duration) motion.duration = time;
                }
                
                size_t numComps = cgltf_num_components(sampler->output->type);
                if (animChannel.interpolation == GltfInterpolationType::CubicSpline)
                {
                    // W specyfikacji glTF 2.0 dla CUBICSPLINE na kazdy punkt czasu przypadaja 3 elementy: in-tangent, value, out-tangent.
                    // Pobieramy wartosc kluczowa z indeksu t * 3 + 1, zachowujac synchronizacje 1:1 z channel.times.
                    animChannel.values.resize(inputCount);
                    for (size_t t = 0; t < inputCount; ++t) {
                        float val[4] = {0, 0, 0, 0};
                        size_t valueIndex = t * 3 + 1;
                        if (valueIndex < sampler->output->count) {
                            cgltf_accessor_read_float(sampler->output, valueIndex, val, numComps);
                        }
                        animChannel.values[t] = {val[0], val[1], val[2], val[3]};
                    }
                }
                else
                {
                    size_t outputCount = sampler->output->count;
                    animChannel.values.resize(outputCount);
                    for (size_t v = 0; v < outputCount; ++v) {
                        float val[4] = {0,0,0,0};
                        cgltf_accessor_read_float(sampler->output, v, val, numComps);
                        animChannel.values[v] = {val[0], val[1], val[2], val[3]};
                    }
                }
            }
            motion.channels.push_back(animChannel);
        }
        outModelData.motions.push_back(motion);
    }
    
    // Metin2 Events (Extract)
    ExtractEvents(data, outModelData);

    return true;
}
