format PE DLL
include 'win32a.inc'
entry DllMain
 
section '.data' data readable writeable
        GetLocalPlayer dd ?
        ModuleName db 'server.dll',0

section '.code' code executable
proc DllMain, hInstance, fdwReason, lpvReserved
    cmp [fdwReason], 0x1
    jnz @just_return_
    invoke getModuleHandle, ModuleName
    add eax, 0x26D5F0
    mov dword[GetLocalPlayer], eax
    invoke createThread, 0, 0, StartRoutine, 0, 0, 0

    @just_return_:
        mov eax,0x1
        ret
endp
 
proc StartRoutine
    jmp @lp

    @sleepc:
        invoke sleep, 3000
    @lp:
        invoke GetLocalPlayer
        test eax,eax
        jz @sleepc

        mov dword[eax+0xDAC], 0x45F30800; 7777.f(new speed value) as hex value (can use cvti2vss or other instruction for normal convertation)

        jmp @sleepc
endp
 
section '.idata' import data readable
library kernel32, 'kernel32.dll'

import kernel32,\
       sleep, 'Sleep',\
       createThread, 'CreateThread',\
       getModuleHandle, 'GetModuleHandleA'
 
section '.reloc' fixups data readable discardable
