public class StringBuilderChars03 {
  public void test(String input) {
    StringBuilder buffer = new StringBuilder(input);
    assert buffer.charAt(0) == buffer.charAt(4);
  }
}
