#include <windows.h>
#include <tlhelp32.h>
#include <dbghelp.h>
#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <sstream>

#pragma comment(lib, "dbghelp.lib")

static bool EnableDebugPrivilege()
{
    HANDLE hToken;
    if (!OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &hToken))
        return false;

    LUID luid;
    if (!LookupPrivilegeValue(nullptr, SE_DEBUG_NAME, &luid))
    {
        CloseHandle(hToken);
        return false;
    }

    TOKEN_PRIVILEGES tp;
    tp.PrivilegeCount = 1;
    tp.Privileges[0].Luid = luid;
    tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

    BOOL ok = AdjustTokenPrivileges(hToken, FALSE, &tp, sizeof(TOKEN_PRIVILEGES), nullptr, nullptr);
    CloseHandle(hToken);
    return ok && GetLastError() == ERROR_SUCCESS;
}

struct ThreadInfo
{
    DWORD tid;
    FILETIME creationTime;
    bool isMain;
};

int main(int argc, char* argv[])
{
    if (argc < 2)
    {
        std::cerr << "Usage: dump_threads.exe <PID> [dump_file_path]" << std::endl;
        return 1;
    }

    DWORD pid = static_cast<DWORD>(std::stoul(argv[1]));
    std::string dumpPath = (argc >= 3) ? argv[2] : "tools/client_deadlock.dmp";

    EnableDebugPrivilege();

    HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);
    if (!hProcess)
    {
        std::cerr << "Failed to open process PID " << pid << " Error: " << GetLastError() << std::endl;
        return 1;
    }

    SymSetOptions(SYMOPT_LOAD_LINES | SYMOPT_UNDNAME | SYMOPT_DEFERRED_LOADS);
    const char* symSearchPath = "E:\\m2dev client backlup\\build\\bin\\Release;E:\\m2dev client backlup\\m2dev-client-main;E:\\m2dev client backlup\\build\\src\\Client\\Release";
    if (!SymInitialize(hProcess, symSearchPath, TRUE))
    {
        std::cerr << "SymInitialize failed: " << GetLastError() << std::endl;
    }

    // Enumerate threads
    std::vector<ThreadInfo> threads;
    HANDLE hSnapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
    if (hSnapshot != INVALID_HANDLE_VALUE)
    {
        THREADENTRY32 te;
        te.dwSize = sizeof(THREADENTRY32);
        if (Thread32First(hSnapshot, &te))
        {
            do
            {
                if (te.th32OwnerProcessID == pid)
                {
                    ThreadInfo info;
                    info.tid = te.th32ThreadID;
                    info.creationTime = {0, 0};
                    info.isMain = false;

                    HANDLE hTh = OpenThread(THREAD_QUERY_INFORMATION, FALSE, te.th32ThreadID);
                    if (hTh)
                    {
                        FILETIME ftExit, ftKernel, ftUser;
                        GetThreadTimes(hTh, &info.creationTime, &ftExit, &ftKernel, &ftUser);
                        CloseHandle(hTh);
                    }
                    threads.push_back(info);
                }
            } while (Thread32Next(hSnapshot, &te));
        }
        CloseHandle(hSnapshot);
    }

    // Sort threads by creation time to identify main thread (earliest)
    if (!threads.empty())
    {
        std::sort(threads.begin(), threads.end(), [](const ThreadInfo& a, const ThreadInfo& b) {
            ULARGE_INTEGER ua, ub;
            ua.LowPart = a.creationTime.dwLowDateTime;
            ua.HighPart = a.creationTime.dwHighDateTime;
            ub.LowPart = b.creationTime.dwLowDateTime;
            ub.HighPart = b.creationTime.dwHighDateTime;
            return ua.QuadPart < ub.QuadPart;
        });
        threads[0].isMain = true;
    }

    std::cout << "{\n";
    std::cout << "  \"pid\": " << pid << ",\n";
    std::cout << "  \"thread_count\": " << threads.size() << ",\n";
    std::cout << "  \"threads\": [\n";

    for (size_t tIdx = 0; tIdx < threads.size(); ++tIdx)
    {
        const auto& tInfo = threads[tIdx];
        HANDLE hThread = OpenThread(THREAD_ALL_ACCESS, FALSE, tInfo.tid);
        if (!hThread)
        {
            hThread = OpenThread(THREAD_GET_CONTEXT | THREAD_SUSPEND_RESUME | THREAD_QUERY_INFORMATION, FALSE, tInfo.tid);
        }

        std::cout << "    {\n";
        std::cout << "      \"tid\": " << tInfo.tid << ",\n";
        std::cout << "      \"is_main\": " << (tInfo.isMain ? "true" : "false") << ",\n";
        std::cout << "      \"frames\": [\n";

        if (hThread)
        {
            SuspendThread(hThread);

            CONTEXT ctx{};
            ctx.ContextFlags = CONTEXT_FULL;
            if (GetThreadContext(hThread, &ctx))
            {
                STACKFRAME64 frame{};
                frame.AddrPC.Offset = ctx.Rip;
                frame.AddrPC.Mode = AddrModeFlat;
                frame.AddrFrame.Offset = ctx.Rbp;
                frame.AddrFrame.Mode = AddrModeFlat;
                frame.AddrStack.Offset = ctx.Rsp;
                frame.AddrStack.Mode = AddrModeFlat;

                alignas(SYMBOL_INFO) char symBuf[sizeof(SYMBOL_INFO) + 512] = {0};
                PSYMBOL_INFO pSym = reinterpret_cast<PSYMBOL_INFO>(symBuf);
                pSym->SizeOfStruct = sizeof(SYMBOL_INFO);
                pSym->MaxNameLen = 511;

                int frameNum = 0;
                while (StackWalk64(
                    IMAGE_FILE_MACHINE_AMD64,
                    hProcess,
                    hThread,
                    &frame,
                    &ctx,
                    nullptr,
                    SymFunctionTableAccess64,
                    SymGetModuleBase64,
                    nullptr))
                {
                    if (frame.AddrPC.Offset == 0)
                        break;

                    DWORD64 disp = 0;
                    std::string symName = "unknown";
                    if (SymFromAddr(hProcess, frame.AddrPC.Offset, &disp, pSym))
                    {
                        symName = pSym->Name;
                    }

                    IMAGEHLP_MODULE64 modInfo{};
                    modInfo.SizeOfStruct = sizeof(IMAGEHLP_MODULE64);
                    std::string modName = "";
                    if (SymGetModuleInfo64(hProcess, frame.AddrPC.Offset, &modInfo))
                    {
                        modName = modInfo.ModuleName;
                    }

                    IMAGEHLP_LINE64 lineInfo{};
                    lineInfo.SizeOfStruct = sizeof(IMAGEHLP_LINE64);
                    DWORD lineDisp = 0;
                    std::string srcFile = "";
                    DWORD lineNum = 0;
                    if (SymGetLineFromAddr64(hProcess, frame.AddrPC.Offset, &lineDisp, &lineInfo))
                    {
                        if (lineInfo.FileName)
                        {
                            srcFile = lineInfo.FileName;
                            for (char& c : srcFile)
                            {
                                if (c == '\\') c = '/';
                            }
                        }
                        lineNum = lineInfo.LineNumber;
                    }

                    if (frameNum > 0) std::cout << ",\n";
                    std::cout << "        {\n";
                    std::cout << "          \"frame\": " << frameNum << ",\n";
                    std::cout << "          \"pc\": \"0x" << std::hex << frame.AddrPC.Offset << std::dec << "\",\n";
                    std::cout << "          \"module\": \"" << modName << "\",\n";

                    // Escape quotes and backslashes in symbol name
                    std::string escapedSym;
                    for (char c : symName)
                    {
                        if (c == '"') escapedSym += "\\\"";
                        else if (c == '\\') escapedSym += "\\\\";
                        else escapedSym += c;
                    }
                    std::cout << "          \"symbol\": \"" << escapedSym << "\",\n";
                    std::cout << "          \"disp\": " << disp << ",\n";
                    std::cout << "          \"file\": \"" << srcFile << "\",\n";
                    std::cout << "          \"line\": " << lineNum << "\n";
                    std::cout << "        }";

                    frameNum++;
                    if (frameNum >= 64) break;
                }
            }

            ResumeThread(hThread);
            CloseHandle(hThread);
        }

        std::cout << "\n      ]\n";
        std::cout << "    }" << ((tIdx + 1 < threads.size()) ? "," : "") << "\n";
    }

    std::cout << "  ]\n";
    std::cout << "}\n";

    // Write MiniDump
    if (!dumpPath.empty())
    {
        HANDLE hFile = CreateFileA(dumpPath.c_str(), GENERIC_WRITE, 0, nullptr, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, nullptr);
        if (hFile != INVALID_HANDLE_VALUE)
        {
            MINIDUMP_TYPE dumpType = static_cast<MINIDUMP_TYPE>(
                MiniDumpWithDataSegs |
                MiniDumpWithIndirectlyReferencedMemory |
                MiniDumpWithProcessThreadData |
                MiniDumpWithThreadInfo |
                MiniDumpWithHandleData |
                MiniDumpWithUnloadedModules
            );

            MiniDumpWriteDump(hProcess, pid, hFile, dumpType, nullptr, nullptr, nullptr);
            CloseHandle(hFile);
            std::cerr << "Minidump written successfully to " << dumpPath << std::endl;
        }
        else
        {
            std::cerr << "Failed to create dump file: " << dumpPath << " Error: " << GetLastError() << std::endl;
        }
    }

    SymCleanup(hProcess);
    CloseHandle(hProcess);
    return 0;
}
