// Option type
fun findUser(id: Int): Option<String> =
  if id == 1 then
    Some("Alice")
  else if id == 2 then
    Some("Bob")
  else
    None

fun main() {
  let user1 = findUser(1) or "Unknown"
  let user2 = findUser(999) or "Unknown"
  print(user1)
  print(user2)
}
