public class StringBuilderChars04 {
  public static void test(StringBuilder buffer) {
    char[] charArray = new char[buffer.length()];
    buffer.getChars(0, buffer.length(), charArray, 0);

    int i = 0;
    for (char character : charArray) {
      System.out.print(character);
      assert character == buffer.charAt(i);
      ++i;
    }
  }
}
