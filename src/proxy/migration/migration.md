Een identifier bestaat uit {id: u64, canister_principal: Principal, kind: String}
Een encoded identifier wordt een Principal

Alle child canisters werken met deze identifiers, bij de proxy willen we hier vanaf en overgaan naar user principals voor user specific data en u64 voor andere objecten.
Object - child - proxy
Profile - identifier - caller
Member - identifier - caller
Attendee - identifier - caller
Group - identifier - u64
Event - identifier - u64
Report - identifier - u64
