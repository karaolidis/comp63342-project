public class StringBuilderChars02 {
  public static void test(String input) {
    StringBuilder buffer = new StringBuilder(input);
    assert buffer.toString().equals("DiffBlue Limitted");
  }
}
