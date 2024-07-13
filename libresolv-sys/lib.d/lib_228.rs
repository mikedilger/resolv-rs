mod resolv;

pub use resolv::{
    MAXDNAME,
    MAXHOSTNAMELEN,
    NS_PACKETSZ,
    RES_DEBUG,
    RES_DEFAULT,
    RES_DEFNAMES,
    RES_DNSRCH,
    RES_IGNTC,
    RES_INIT,
    RES_NOALIASES,
    RES_NORELOAD,
    RES_NOTLDQUERY,
    RES_RECURSE,
    RES_ROTATE,
    RES_SNGLKUP,
    RES_SNGLKUPREOP,
    RES_STAYOPEN,
    RES_USEVC,
    RES_USE_DNSSEC,
    RES_USE_EDNS0,
    __ns_sect_ns_s_an,
    __ns_sect_ns_s_ar,
    __ns_sect_ns_s_ns,
    __ns_sect_ns_s_qd,
    __res_ninit as res_ninit,
    __res_nquery as res_nquery,
    __res_nsearch as res_nsearch,
    __res_state,
    ns_initparse,
    ns_msg,
    ns_name_uncompress,
    ns_parserr,
    ns_rr,
    ns_sect,
};

/// Options for the Resolver
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolverOption {
    /// address initialized
    Init = RES_INIT,
    /// print debug messages
    Debug = RES_DEBUG,
    /// use virtual circuit
    UseVC = RES_USEVC,
    /// ignore truncation errors
    IgnTc = RES_IGNTC,
    /// recursion desired
    Recurse = RES_RECURSE,
    /// use default domain name
    DefNames = RES_DEFNAMES,
    /// Keep TCP socket open
    StayOpen = RES_STAYOPEN,
    /// search up local domain tree
    DNSrch = RES_DNSRCH,
    /// shuts off HOSTALIASES feature
    NoAliases = RES_NOALIASES,
    /// rotate ns list after each query
    Rotate = RES_ROTATE,
    /// Use EDNS0.
    UseEDNS0 = RES_USE_EDNS0,
    /// one outstanding request at a time
    SngLkup = RES_SNGLKUP,
    /// one outstanding request at a time, but open new socket for each request
    SngLkupReop = RES_SNGLKUPREOP,
    /// use DNSSEC using OK bit in OPT
    UseDNSSEC = RES_USE_DNSSEC,
    /// Do not look up unqualified name as a TLD.
    NoTLDQuery = RES_NOTLDQUERY,
    /// No automatic configuration reload (since glibc 2.26; invalid in prior versions)
    NoReload = RES_NORELOAD,
    /// Default values
    Default = RES_DEFAULT,
}
