

#![ allow (unused_imports) ]




// NOTE:  #>  python -c $'print ("macro_rules! __count_list {")\nfor n in range (1, 512 + 1) :\n  for e in range (1, 16 + 1) :\n    if e <= n : print ("( %d, %d )" % (n, e) + "=> { [ " + ", ".join (["%d" % c for c in range (0, n + 1, e) if c != 0]) + ", ] };")\nprint ("}")' >| ./sources/_generated/patterns_count_list.in
include! ("./macros_count_list.in");


// NOTE:  #>  python -c $'print ("macro_rules! __count_call_each {")\nfor n in range (1, 512 + 1) :\n  for e in range (1, 16 + 1) :\n    if e <= n : print ("( [ %d : %d ] => $c:ident! ( $($p:tt)* ) )" % (n, e) + "=> {\\n" + "\\n".join (["\t$c! ( $($p)* %d );" % c for c in range (0, n + 1, e) if c != 0]) + "\\n};")\nprint ("}")' >| ./sources/_generated/patterns_count_call_each.in
include! ("./macros_count_call_each.in");


// NOTE:  #>  python -c $'print ("macro_rules! __count_call_with {")\nfor n in range (1, 512 + 1) :\n  for e in range (1, 16 + 1) :\n    if e <= n : print ("( [ %d : %d ] => $c:ident! ( $($p:tt)* ) )" % (n, e) + "=> { $c! ( $($p)* [ " + ", ".join (["%d" % c for c in range (0, n + 1, e) if c != 0]) + ", ] ); };")\nprint ("}")' >| ./sources/_generated/patterns_count_call_with.in
include! ("./macros_count_call_with.in");




pub(crate) use __count_list;
pub(crate) use __count_call_each;
pub(crate) use __count_call_with;


