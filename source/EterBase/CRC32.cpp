#include "StdAfx.h"
#include "CRC32.h"
#include "m2_client_bridge.h"

DWORD GetCRC32(const char * buf, size_t len)
{
	if (!buf || len == 0)
		return 0;
	return m2_crc32((const uint8_t*)buf, len);
}

DWORD GetCaseCRC32(const char * buf, size_t len)
{
	if (!buf || len == 0)
		return 0;
	return m2_case_crc32((const uint8_t*)buf, len);
}

DWORD GetHFILECRC32(HANDLE hFile)
{
	DWORD dwRetCRC32 = 0;
	DWORD dwFileSize = GetFileSize(hFile, NULL);
	if (dwFileSize == 0 || dwFileSize == INVALID_FILE_SIZE)
		return 0;

	DWORD dataOffset = 0;
	DWORD mapSize = dwFileSize;

	SYSTEM_INFO SysInfo;
	GetSystemInfo(&SysInfo);
	
	DWORD dwSysGran = SysInfo.dwAllocationGranularity;
	DWORD dwFileMapStart = (dataOffset / dwSysGran) * dwSysGran;
	DWORD dwMapViewSize = (dataOffset % dwSysGran) + mapSize;

	HANDLE hFM = CreateFileMapping(hFile, NULL, PAGE_READONLY, 0, dataOffset + mapSize, NULL);
	if (hFM)
	{	
		LPVOID lpMapData = MapViewOfFile(hFM, FILE_MAP_READ, 0, dwFileMapStart, dwMapViewSize);
		if (lpMapData)
		{
			dwRetCRC32 = GetCRC32((const char*)lpMapData, dwFileSize);
			UnmapViewOfFile(lpMapData);
		}
		CloseHandle(hFM);
	}

	return dwRetCRC32;
}

DWORD GetFileCRC32(const char* c_szFileName)
{
	HANDLE hFile = CreateFile(c_szFileName, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
	if (INVALID_HANDLE_VALUE == hFile)
		return 0;

	DWORD dwRetCRC32 = GetHFILECRC32(hFile);
	CloseHandle(hFile);
	return dwRetCRC32;
}

DWORD GetFileSize(const char* c_szFileName)
{
	HANDLE hFile = CreateFile(c_szFileName, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
	if (INVALID_HANDLE_VALUE == hFile)
		return 0;

	DWORD dwSize = GetFileSize(hFile, NULL);
	CloseHandle(hFile);
	return dwSize;
}
