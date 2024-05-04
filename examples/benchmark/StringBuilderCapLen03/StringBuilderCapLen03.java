public class StringBuilderCapLen03 {
  public static void test(String nondetString) {
    StringBuilder buffer = new StringBuilder(nondetString);
    assert buffer.length() == 51;
  }
}
