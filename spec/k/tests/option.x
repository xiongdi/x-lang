// Option type
function findUser(id: integer 32): Option<string> =
  if id == 1 then
    Some("Alice")
  else if id == 2 then
    Some("Bob")
  else
    None

function main() {
  let user1 = findUser(1) or "Unknown"
  let user2 = findUser(999) or "Unknown"
  print(user1)
  print(user2)
}
