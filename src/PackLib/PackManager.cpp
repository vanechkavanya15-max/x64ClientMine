#include "PackManager.h"
#include "EterLib/BufferPool.h"
#include <fstream>
#include <filesystem>
#include <algorithm>
#include "EterBase/Debug.h"

#ifdef _WIN32
#include <windows.h>
#endif

static std::string PathToGenericUtf8String(const std::filesystem::path& p)
{
	std::wstring w = p.generic_wstring();
	if (w.empty())
		return {};
#ifdef _WIN32
	int sizeNeeded = WideCharToMultiByte(CP_UTF8, 0, w.c_str(), static_cast<int>(w.size()), nullptr, 0, nullptr, nullptr);
	if (sizeNeeded <= 0)
		return {};
	std::string result(sizeNeeded, '\0');
	WideCharToMultiByte(CP_UTF8, 0, w.c_str(), static_cast<int>(w.size()), &result[0], sizeNeeded, nullptr, nullptr);
	return result;
#else
	return p.generic_string();
#endif
}

static std::string StripDrivePrefix(std::string_view p)
{
	// Usun wiodace ukosniki
	while (!p.empty() && (p.front() == '/' || p.front() == '\\')) {
		p.remove_prefix(1);
	}

	// Usun prefiks dysku (np. "d:" lub "C:")
	if (p.size() >= 2 && std::isalpha(static_cast<unsigned char>(p[0])) && p[1] == ':') {
		p.remove_prefix(2);
		while (!p.empty() && (p.front() == '/' || p.front() == '\\')) {
			p.remove_prefix(1);
		}
	}

	// Blokada sekwencji '..' oraz pozostalych dwukropkow (Path Traversal i ADS)
	if (p.find("..") != std::string_view::npos || p.find(':') != std::string_view::npos) {
		return {};
	}

	return std::string(p);
}

CPackManager::CPackManager()
	: m_load_from_pack(true)
	, m_pBufferPool(nullptr)
{
	m_pBufferPool = new CBufferPool();
}

CPackManager::~CPackManager()
{
	if (m_pBufferPool)
	{
		delete m_pBufferPool;
		m_pBufferPool = nullptr;
	}
}

bool CPackManager::AddPack(const std::string& path)
{
	std::shared_ptr<CPack> pack = std::make_shared<CPack>();

	if (!pack->Load(path))
	{
		return false;
	}

	std::unique_lock<std::shared_mutex> lock(m_mutex);

	// Rejestracja nazwy paczki
	std::string packName = pack->GetPackName();
	if (!packName.empty())
	{
		if (std::find(m_packNames.begin(), m_packNames.end(), packName) == m_packNames.end())
		{
			m_packNames.push_back(packName);
		}
	}

	// Detekcja katalogu bazowego dla assets/
	if (m_assetBasePath.empty())
	{
		std::error_code ec;
		std::filesystem::path p = std::filesystem::u8path(path);
		std::filesystem::path cand = p.parent_path().parent_path() / "assets";
		if (std::filesystem::exists(cand, ec) && std::filesystem::is_directory(cand, ec))
		{
			auto can = std::filesystem::canonical(cand, ec);
			m_assetBasePath = ec ? PathToGenericUtf8String(cand) : PathToGenericUtf8String(can);
		}
		else if (std::filesystem::exists("assets", ec) && std::filesystem::is_directory("assets", ec))
		{
			auto can = std::filesystem::canonical("assets", ec);
			m_assetBasePath = ec ? "assets" : PathToGenericUtf8String(can);
		}
		else if (std::filesystem::exists("m2dev-client-main/assets", ec) && std::filesystem::is_directory("m2dev-client-main/assets", ec))
		{
			auto can = std::filesystem::canonical("m2dev-client-main/assets", ec);
			m_assetBasePath = ec ? "m2dev-client-main/assets" : PathToGenericUtf8String(can);
		}
		else if (std::filesystem::exists("../../m2dev-client-main/assets", ec) && std::filesystem::is_directory("../../m2dev-client-main/assets", ec))
		{
			auto can = std::filesystem::canonical("../../m2dev-client-main/assets", ec);
			m_assetBasePath = ec ? "../../m2dev-client-main/assets" : PathToGenericUtf8String(can);
		}
	}

	const auto& index = pack->GetIndex();
	std::string normKey;
	for (const auto& entry : index)
	{
		NormalizePath(entry.file_name, normKey);
		if (!normKey.empty())
		{
			m_entries[normKey] = std::make_pair(pack, entry);
		}
	}

	return true;
}

static bool DiskFileExists(const std::string& fullPath)
{
	if (fullPath.empty())
		return false;
#ifdef _WIN32
	int wlen = MultiByteToWideChar(CP_UTF8, 0, fullPath.c_str(), static_cast<int>(fullPath.size()), nullptr, 0);
	if (wlen <= 0)
		return false;
	std::wstring wpath(wlen, L'\0');
	MultiByteToWideChar(CP_UTF8, 0, fullPath.c_str(), static_cast<int>(fullPath.size()), &wpath[0], wlen);
	DWORD attr = GetFileAttributesW(wpath.c_str());
	return (attr != INVALID_FILE_ATTRIBUTES && !(attr & FILE_ATTRIBUTE_DIRECTORY));
#else
	std::error_code ec;
	return std::filesystem::is_regular_file(std::filesystem::u8path(fullPath), ec);
#endif
}

void CPackManager::EnsureAssetIndexBuilt() const
{
	{
		std::shared_lock<std::shared_mutex> lock(m_mutex);
		if (m_assetIndexBuilt)
			return;
	}

	std::unique_lock<std::shared_mutex> lock(m_mutex);
	if (m_assetIndexBuilt)
		return;

	if (m_assetBasePath.empty())
	{
		std::error_code ec;
		if (std::filesystem::exists("assets", ec) && std::filesystem::is_directory("assets", ec))
		{
			auto can = std::filesystem::canonical("assets", ec);
			m_assetBasePath = ec ? "assets" : PathToGenericUtf8String(can);
		}
		else if (std::filesystem::exists("m2dev-client-main/assets", ec) && std::filesystem::is_directory("m2dev-client-main/assets", ec))
		{
			auto can = std::filesystem::canonical("m2dev-client-main/assets", ec);
			m_assetBasePath = ec ? "m2dev-client-main/assets" : PathToGenericUtf8String(can);
		}
		else if (std::filesystem::exists("../../m2dev-client-main/assets", ec) && std::filesystem::is_directory("../../m2dev-client-main/assets", ec))
		{
			auto can = std::filesystem::canonical("../../m2dev-client-main/assets", ec);
			m_assetBasePath = ec ? "../../m2dev-client-main/assets" : PathToGenericUtf8String(can);
		}
	}

	m_assetIndexBuilt = true;
}

std::string CPackManager::ResolveDiskPath(const std::string& clean, const std::string& preferredPack) const
{
	if (clean.empty())
		return {};

	EnsureAssetIndexBuilt();

	if (m_assetBasePath.empty())
		return {};

	std::string cacheKey = preferredPack.empty() ? clean : (preferredPack + "#" + clean);

	{
		std::shared_lock<std::shared_mutex> lock(m_mutex);
		auto it = m_diskPathCache.find(cacheKey);
		if (it != m_diskPathCache.end())
		{
			return it->second;
		}
	}

	std::string resolvedPath;

	// 1. assets/<clean>
	std::string cand1 = m_assetBasePath + "/" + clean;
	if (DiskFileExists(cand1))
	{
		resolvedPath = cand1;
	}

	// 2. assets/<preferredPack>/<clean>
	if (resolvedPath.empty() && !preferredPack.empty())
	{
		std::string cand2 = m_assetBasePath + "/" + preferredPack + "/" + clean;
		if (DiskFileExists(cand2))
		{
			resolvedPath = cand2;
		}
	}

	// 3. assets/<pack>/<clean>
	if (resolvedPath.empty())
	{
		std::vector<std::string> packNames;
		{
			std::shared_lock<std::shared_mutex> lock(m_mutex);
			packNames = m_packNames;
		}
		for (auto rit = packNames.rbegin(); rit != packNames.rend(); ++rit)
		{
			if (*rit == preferredPack)
				continue;

			std::string cand3 = m_assetBasePath + "/" + *rit + "/" + clean;
			if (DiskFileExists(cand3))
			{
				resolvedPath = cand3;
				break;
			}
		}
	}

	std::unique_lock<std::shared_mutex> lock(m_mutex);
	m_diskPathCache[cacheKey] = resolvedPath;
	return resolvedPath;
}

bool CPackManager::GetFile(std::string_view path, TPackFile& result)
{
	return GetFileWithPool(path, result, m_pBufferPool);
}

bool CPackManager::GetFileWithPool(std::string_view path, TPackFile& result, CBufferPool* pPool)
{
	thread_local std::string buf;
	NormalizePath(path, buf);
	if (buf.empty())
		return false;

	auto tryLoadDisk = [&](const std::string& diskPath) -> bool {
		if (diskPath.empty())
			return false;

		std::error_code ec;
#ifdef _WIN32
		int wlen = MultiByteToWideChar(CP_UTF8, 0, diskPath.c_str(), static_cast<int>(diskPath.size()), nullptr, 0);
		std::wstring wdisk(wlen, L'\0');
		if (wlen > 0)
			MultiByteToWideChar(CP_UTF8, 0, diskPath.c_str(), static_cast<int>(diskPath.size()), &wdisk[0], wlen);
		std::filesystem::path fspath(wdisk);
#else
		std::filesystem::path fspath = std::filesystem::u8path(diskPath);
#endif
		if (std::filesystem::exists(fspath, ec) && std::filesystem::is_regular_file(fspath, ec)) {
			std::ifstream ifs(fspath, std::ios::binary);
			if (ifs.is_open()) {
				ifs.seekg(0, std::ios::end);
				size_t size = static_cast<size_t>(ifs.tellg());
				ifs.seekg(0, std::ios::beg);

				if (pPool) {
					result = pPool->Acquire(size);
					result.resize(size);
				} else {
					result.resize(size);
				}

				if (ifs.read(reinterpret_cast<char*>(result.data()), size)) {
					return true;
				}
			}
		}
		return false;
	};

	auto tryLoad = [&](const std::string& targetPath, const std::string& preferredPack = "") -> bool {
		// 1. Sprawdz wpisy w paczkach VFS
		if (m_load_from_pack) {
			std::shared_ptr<CPack> targetPack;
			TPackFileEntry targetEntry;
			bool found = false;
			{
				std::shared_lock<std::shared_mutex> lock(m_mutex);
				auto it = m_entries.find(targetPath);
				if (it != m_entries.end()) {
					targetPack = it->second.first;
					targetEntry = it->second.second;
					found = true;
				}
			}
			if (found && targetPack) {
				return targetPack->GetFileWithPool(targetEntry, result, pPool);
			}
		}

		// 2. Sciezki na dysku (loose files i zmigrowane zasoby glTF)
		std::string clean = StripDrivePrefix(targetPath);
		if (clean.empty())
			return false;

		std::string diskPath = ResolveDiskPath(clean, preferredPack);
		if (!diskPath.empty()) {
			return tryLoadDisk(diskPath);
		}

		return false;
	};

	auto getPreferredPack = [&](const std::string& key) -> std::string {
		std::shared_lock<std::shared_mutex> lock(m_mutex);
		auto it = m_entries.find(key);
		if (it != m_entries.end() && it->second.first) {
			return it->second.first->GetPackName();
		}
		return {};
	};

	// 1. Transparent migration: gdy proszony jest format .gr2, preferuj .glb, ale w razie braku laduj oryginalny .gr2
	if (buf.size() > 4 && buf.ends_with(".gr2")) {
		std::string preferredPack = getPreferredPack(buf);
		std::string glbPath = buf.substr(0, buf.size() - 4) + ".glb";
		if (tryLoad(glbPath, preferredPack))
			return true;
		return tryLoad(buf, preferredPack);
	}

	// 2. Gdy proszony jest .glb, laduj .glb, a w razie braku fallback do .gr2
	if (buf.size() > 4 && buf.ends_with(".glb")) {
		std::string preferredPack = getPreferredPack(buf);
		if (tryLoad(buf, preferredPack))
			return true;
		std::string gr2Path = buf.substr(0, buf.size() - 4) + ".gr2";
		return tryLoad(gr2Path, preferredPack);
	}

	// 3. Transparent fallback dla tekstur (.dds <-> .tga <-> .png <-> .jpg)
	static const std::string_view s_texExts[] = { ".dds", ".tga", ".png", ".jpg" };
	std::string_view curTexExt;
	for (auto ext : s_texExts) {
		if (buf.size() > ext.size() && buf.ends_with(ext)) {
			curTexExt = ext;
			break;
		}
	}

	std::string preferredPack = getPreferredPack(buf);

	if (!curTexExt.empty()) {
		if (tryLoad(buf, preferredPack))
			return true;
		std::string baseWithoutExt = buf.substr(0, buf.size() - curTexExt.size());
		for (auto ext : s_texExts) {
			if (ext == curTexExt)
				continue;
			std::string candidate = baseWithoutExt + std::string(ext);
			if (tryLoad(candidate, preferredPack))
				return true;
		}
		return false;
	}

	// 4. Pozostale pliki (.txt, .py, .mse itp.)
	return tryLoad(buf, preferredPack);
}

bool CPackManager::IsExist(std::string_view path) const
{
	thread_local std::string buf;
	NormalizePath(path, buf);
	if (buf.empty())
		return false;

	auto getPreferredPack = [&](const std::string& key) -> std::string {
		std::shared_lock<std::shared_mutex> lock(m_mutex);
		auto it = m_entries.find(key);
		if (it != m_entries.end() && it->second.first) {
			return it->second.first->GetPackName();
		}
		return {};
	};

	auto checkExist = [&](const std::string& targetPath, const std::string& preferredPack = "") -> bool {
		if (m_load_from_pack) {
			std::shared_lock<std::shared_mutex> lock(m_mutex);
			if (m_entries.find(targetPath) != m_entries.end())
				return true;
		}

		std::string clean = StripDrivePrefix(targetPath);
		if (clean.empty())
			return false;

		std::string diskPath = ResolveDiskPath(clean, preferredPack);
		return !diskPath.empty();
	};

	// 1. Transparent migration check dla .gr2: preferuj .glb, w razie braku sprawdz .gr2
	if (buf.size() > 4 && buf.ends_with(".gr2")) {
		std::string preferredPack = getPreferredPack(buf);
		std::string glbPath = buf.substr(0, buf.size() - 4) + ".glb";
		if (checkExist(glbPath, preferredPack))
			return true;
		return checkExist(buf, preferredPack);
	}

	// 2. Transparent migration check dla .glb: sprawdz .glb, w razie braku sprawdz .gr2
	if (buf.size() > 4 && buf.ends_with(".glb")) {
		std::string preferredPack = getPreferredPack(buf);
		if (checkExist(buf, preferredPack))
			return true;
		std::string gr2Path = buf.substr(0, buf.size() - 4) + ".gr2";
		return checkExist(gr2Path, preferredPack);
	}

	// 3. Transparent fallback dla tekstur (.dds <-> .tga <-> .png <-> .jpg)
	static const std::string_view s_texExts[] = { ".dds", ".tga", ".png", ".jpg" };
	std::string_view curTexExt;
	for (auto ext : s_texExts) {
		if (buf.size() > ext.size() && buf.ends_with(ext)) {
			curTexExt = ext;
			break;
		}
	}

	std::string preferredPack = getPreferredPack(buf);

	if (!curTexExt.empty()) {
		if (checkExist(buf, preferredPack))
			return true;
		std::string baseWithoutExt = buf.substr(0, buf.size() - curTexExt.size());
		for (auto ext : s_texExts) {
			if (ext == curTexExt)
				continue;
			std::string candidate = baseWithoutExt + std::string(ext);
			if (checkExist(candidate, preferredPack))
				return true;
		}
		return false;
	}

	return checkExist(buf, preferredPack);
}

void CPackManager::NormalizePath(std::string_view in, std::string& out) const
{
	out.clear();
	if (in.empty())
		return;

	// 1. Zablokuj sekwencje '..' (Path Traversal)
	if (in.find("..") != std::string_view::npos)
		return;

	// 2. Weryfikacja znakow niedozwolonych
	// Niedozwolone znaki kontrolne (<32, 127) oraz *, ?, ", <, >, |
	// Znak dwukropka ':' moze wystapic tylko jako litera dysku na pozycji 1 (np. "d:/")
	for (size_t i = 0; i < in.size(); ++i) {
		unsigned char c = static_cast<unsigned char>(in[i]);
		if (c < 32 || c == 127)
			return;

		if (c == '*' || c == '?' || c == '"' || c == '<' || c == '>' || c == '|')
			return;

		if (c == ':') {
			if (i != 1 || !std::isalpha(static_cast<unsigned char>(in[0])))
				return;
		}
	}

	out.reserve(in.size());
	bool lastWasSlash = false;
	size_t startIdx = 0;

	// Usun wiodace ukosniki, jesli sciezka nie zaczyna sie od litery dysku
	if (!(in.size() >= 2 && std::isalpha(static_cast<unsigned char>(in[0])) && in[1] == ':')) {
		while (startIdx < in.size() && (in[startIdx] == '/' || in[startIdx] == '\\')) {
			++startIdx;
		}
	}

	for (size_t i = startIdx; i < in.size(); ++i) {
		char c = in[i];
		if (c == '\\')
			c = '/';
		else
			c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));

		if (c == '/') {
			if (lastWasSlash)
				continue;
			lastWasSlash = true;
		} else {
			lastWasSlash = false;
		}
		out.push_back(c);
	}

	if (out.size() > 3 && out.back() == '/') {
		out.pop_back();
	}
}
