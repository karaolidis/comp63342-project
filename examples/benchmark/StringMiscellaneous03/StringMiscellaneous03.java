public class StringMiscellaneous03 {
  public static void test(String s1, String s2) {
    char[] charArray = new char[5];
    int i = 0;
    for (int count = s1.length() - 1; count >= 0; count--) {
      assert s1.charAt(count) != s2.charAt(i);
      ++i;
    }
  }
}
