public class StringValueOf03 {
  public static void test(String nondetString) {
    String[] args = new String[1];
    args[0] = nondetString;
    char[] charArray = {
      args[0].charAt(0), args[0].charAt(1), args[0].charAt(2),
      args[0].charAt(3), args[0].charAt(4), args[0].charAt(5),
      args[0].charAt(6), args[0].charAt(7)
    };
    String tmp = String.valueOf(charArray, 3, 3);
    assert tmp.equals("fbbl");
  }
}
