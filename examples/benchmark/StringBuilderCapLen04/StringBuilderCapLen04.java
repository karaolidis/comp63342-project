public class StringBuilderCapLen04 {
  public static void test(String generatedString) {
    StringBuilder buffer = new StringBuilder(generatedString);
    assert buffer.capacity() == 69;
  }
}
