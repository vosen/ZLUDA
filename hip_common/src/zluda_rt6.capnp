@0xb631f7be1b968e02;

struct Metadata {
    # We can turn it into an union later: "a field can be replaced with a group
    # or union containing an equivalent field and some new fields"
    version1 @0 :Version1;
}

struct Version1 {
    attributesSize @0 :UInt32;
    attributesAlign @1 :UInt32;
    attributes @2 :List(Attribute);
    variablesSize @3 :UInt32;
    variablesAlign @4 :UInt32;
    variables @5 :List(GlobalVariable);
    isCallable @6 :Bool;
}

struct Attribute {
    name @0 :Text;
    offset @1 :UInt32;
    size @2 :UInt32;
}

struct GlobalVariable {
    name @0 :Text;
    offset @1 :UInt32;
    size @2 :UInt32;
    defaultValue @3 :Data;
}
