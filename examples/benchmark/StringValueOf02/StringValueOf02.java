public class StringValueOf02 {
  public static void test(String nondetString) {
    String arg = nondetString;
    char[] charArray = {
      arg.charAt(0), arg.charAt(1), arg.charAt(2),
      arg.charAt(3), arg.charAt(4), arg.charAt(5),
      arg.charAt(6), arg.charAt(7)
    };
    String tmp = String.valueOf(charArray);
    assert tmp.equals("diffblue");
  }
}
