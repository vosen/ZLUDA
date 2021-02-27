@0xbefb36a5417c8ae1;

struct Metadata {
    # We can turn it into an union later: "a field can be replaced with a group
    # or union containing an equivalent field and some new fields"
    version1 @0 :Version1;
}

struct Version1 {
    kernels @0 :List(Kernel);
    smVersion @1 :UInt32;
}

struct Kernel {
    name @0 :Text;
    minGoupSize @1 :UInt32;
    maxGroupSize @2 :UInt32;
}
