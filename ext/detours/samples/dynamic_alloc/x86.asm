;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;
;;  Detours Test Program
;;
;;  Microsoft Research Detours Package
;;
;;  Copyright (c) Microsoft Corporation.  All rights reserved.
;;
.386
.model flat,C

PUBLIC CodeTemplate
PUBLIC CodeTemplate_End

_TEXT SEGMENT

CodeTemplate PROC
  nop
  nop
  nop
  mov eax, 0deadbeefh
  nop
  nop
  nop
  ret
CodeTemplate_End::
CodeTemplate ENDP

_TEXT ENDS

END
