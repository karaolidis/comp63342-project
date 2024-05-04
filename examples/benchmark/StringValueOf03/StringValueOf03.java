public class StringValueOf03 {
  public static void test(String nondetString) {
    char[] charArray = {
      nondetString.charAt(0), nondetString.charAt(1), nondetString.charAt(2),
      nondetString.charAt(3), nondetString.charAt(4), nondetString.charAt(5),
      nondetString.charAt(6), nondetString.charAt(7)
    };
    String tmp = String.valueOf(charArray, 3, 3);
    assert tmp.equals("fbbl");
  }
}
