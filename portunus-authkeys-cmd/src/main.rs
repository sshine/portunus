//! # `portunus-authkeys-cmd`
//!
//! ### `AuthorizedKeysCommand`
//!
//! ```norun
//! Specifies  a  program  to  be  used to look up the user's public keys.  The program must be owned by
//! root,  not  writable  by  group  or  others  and  specified  by  an  absolute  path.   Arguments  to
//! AuthorizedKeysCommand  accept  the  tokens  described  in the “TOKENS” section.  If no arguments are
//! specified then the username of the target user is used.
//!
//! The program should produce on standard output zero or more  lines  of  authorized_keys  output  (see
//! “AUTHORIZED_KEYS”  in  sshd(8)).   AuthorizedKeysCommand is tried after the usual AuthorizedKeysFile
//! files  and  will  not  be  executed  if  a  matching  key  is   found   there.    By   default,   no
//! AuthorizedKeysCommand is run.
//!
//! AuthorizedKeysCommandUser
//! Specifies the user under whose account the AuthorizedKeysCommand is run.  It is recommended to use a
//! dedicated  user  that  has  no  other  role  on  the host than running authorized keys commands.  If
//! AuthorizedKeysCommand is specified but AuthorizedKeysCommandUser is not, then sshd(8) will refuse to
//! start.
//! ```

fn main() {
    println!("portunus-authkeys-cmd");
}
