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
//!
//! ### TOKENS
//!
//! ```norun
//! Arguments to some keywords can make use of tokens, which are expanded at runtime:
//!
//! %%    A literal ‘%’.
//! %C    Identifies the connection endpoints, containing four  space-separated  values:  client  address,
//!       client port number, server address, and server port number.
//! %D    The routing domain in which the incoming connection was received.
//! %F    The fingerprint of the CA key.
//! %f    The fingerprint of the key or certificate.
//! %h    The home directory of the user.
//! %i    The key ID in the certificate.
//! %K    The base64-encoded CA key.
//! %k    The base64-encoded key or certificate for authentication.
//! %s    The serial number of the certificate.
//! %T    The type of the CA key.
//! %t    The key or certificate type.
//! %U    The numeric user ID of the target user.
//! %u    The username.
//!
//! AuthorizedKeysCommand accepts the tokens %%, %C, %D, %f, %h, %k, %t, %U, and %u.
//!
//! [...]
//! ```

fn main() {
    println!("portunus-authkeys-cmd");
    let mut args = std::env::args_os();
    let _executable = args.next();
    match args.next() {
        Some(_user) => println!("You're somebody. {:?}", args),
        None => println!("You're nobody?"),
    }
}
