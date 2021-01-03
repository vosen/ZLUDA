;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;
;;  Detours Test Program
;;
;;  Microsoft Research Detours Package
;;
;;  Copyright (c) Microsoft Corporation.  All rights reserved.
;;
PUBLIC CodeTemplate
PUBLIC CodeTemplate_End

_TEXT SEGMENT

CodeTemplate PROC
  nop
  nop
  mov rax, 0deadbeef00000000h
  nop
  ret
CodeTemplate_End::
CodeTemplate ENDP

_TEXT ENDS

END
