public class StringBuilderChars06 {
  public static void test(String input) {
    StringBuilder buffer = new StringBuilder(input);
    buffer.reverse();
    assert buffer.toString().equals("detimiL eTlBffiiH");
  }
}
