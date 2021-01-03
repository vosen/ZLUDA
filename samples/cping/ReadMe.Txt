Microsoft Research Detours Package
==============================================================================
4/2/98

* Instrumentation:
    Read Pentium cycle counter

* PC configuration:
    DCOM/TCP, Windows NT Server 4.0,
    between two 300MHz Pentium boxes,
    Ethernet connecction

* Client test program:
    HRESULT get(SHORT, SHORT, LONG*)
    average over 1,000 calls
    midl /Oicf

* Results:
    get() {
                    <-- (1)
      IRpcChannelBuffer::SendReceive()) {
                    <-- (2)
        I_RpcSendReceive() {
                    <-- (3)
          send(soc, )
                    <-- (4)
          NtWaitForSingleObject(soc, )
                    <-- (5)
        } // end of RPC layer
                    <-- (6)
      } // end of channel object
                    <-- (7)
    } // end of client call
                            Average number
                               of Pentium cycles
    (1) NDR marshaling overhead (2 SHORTs)      13   K
                (No! of which 11K from GetBuffer,
                     of which 6.2K from I_RpcGetBuffer()!)
    (2) Channel object one-way (send) overhead       1.0 K
    (3) RPC layer one-way (send) overhead        5.3 K
    (4) TCP + all server work                      200   K
    (5) RPC layer one-way (recv) overhead        5.1 K
    (6) Channel object one-way (recv) overhead       2.2 K
    (7) NDR unmarshaling overhead (2 LONGs)      4.2 K

    (*) send() only                 17   K
    TOTAL CYCLES for client get(): 230 K
