public class StringBuilderCapLen03 {
  public static void test(String input) {
    StringBuilder buffer = new StringBuilder(input);
    assert buffer.length() == 51;
  }
}
