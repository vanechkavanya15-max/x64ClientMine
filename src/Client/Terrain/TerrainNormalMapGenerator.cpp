#include "TerrainNormalMapGenerator.h"

#include <cmath>
#include <new> // For std::bad_alloc

namespace Client::Terrain {

std::expected<std::vector<Client::Core::MapCoords>, NormalMapError> TerrainNormalMapGenerator::Generate(
    uint32_t width, 
    uint32_t height, 
    std::span<const float> heights) const noexcept
{
    if (width < 2 || height < 2)
    {
        return std::unexpected(NormalMapError::InvalidGridSize);
    }

    if (heights.size() != static_cast<size_t>(width) * height)
    {
        return std::unexpected(NormalMapError::InvalidSpanSize);
    }

    std::vector<Client::Core::MapCoords> normals;
    
    try 
    {
        normals.resize(static_cast<size_t>(width) * height);
    } 
    catch (const std::bad_alloc&) 
    {
        return std::unexpected(NormalMapError::AllocationFailed);
    }

    // Obliczamy normale za pomoca algorytmu roznicy centralnej (central difference)
    for (uint32_t y = 0; y < height; ++y)
    {
        for (uint32_t x = 0; x < width; ++x)
        {
            // Obsluga krawedzi poprzez powielenie sasiadujacych wartosci wysokosci (clamping)
            const uint32_t x_minus = (x > 0) ? (x - 1) : 0;
            const uint32_t x_plus = (x < width - 1) ? (x + 1) : (width - 1);
            
            const uint32_t y_minus = (y > 0) ? (y - 1) : 0;
            const uint32_t y_plus = (y < height - 1) ? (y + 1) : (height - 1);

            // Mnozniki odleglosci - dla krawedzi uzywamy 1.0f (poniewaz duplikujemy sasiada, wiec odleglosc logiczna w tej osi to 1 unit gridu), 
            // dla srodka 2.0f
            const float dx = (x > 0 && x < width - 1) ? 2.0f : 1.0f;
            const float dy = (y > 0 && y < height - 1) ? 2.0f : 1.0f;

            // Pobieramy wysokosci sasiadow
            const float h_left = heights[y * width + x_minus];
            const float h_right = heights[y * width + x_plus];
            const float h_down = heights[y_minus * width + x];
            const float h_up = heights[y_plus * width + x];

            // Pochodne czastkowe
            const float dfdx = (h_right - h_left) / dx;
            const float dfdy = (h_up - h_down) / dy; // W ukladzie gdzie +Z to wysokosc, a X i Y to osie poziome

            // Wektor normalny [-dfdx, -dfdy, 1.0f] przed normalizacja
            Client::Core::MapCoords normal = { -dfdx, -dfdy, 1.0f };
            
            // Normalizacja
            const float lengthSquared = (normal.x * normal.x) + (normal.y * normal.y) + (normal.z * normal.z);
            if (lengthSquared > 0.000001f)
            {
                const float inverseLength = 1.0f / std::sqrt(lengthSquared);
                normal.x *= inverseLength;
                normal.y *= inverseLength;
                normal.z *= inverseLength;
            }
            else
            {
                normal.x = 0.0f;
                normal.y = 0.0f;
                normal.z = 1.0f;
            }

            normals[y * width + x] = normal;
        }
    }

    return normals;
}

} // namespace Client::Terrain
