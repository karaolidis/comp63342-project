public class StringConstructors05 {
  public static void test(char[] charArray, String nondetString) {
    String s3 = new String(charArray);
    assert s3.equals(nondetString);
  }
}
