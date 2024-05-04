public class StringMiscellaneous03 {
  public void test(String s1, String s2) {
    if (s1 == null || s2 == null || s1.length() != s2.length()) {
      throw new AssertionError("Input strings must not be null and must have the same length.");
    }
    char[] charArray = new char[5];
    int i = 0;
    for (int count = s1.length() - 1; count >= 0; count--) {
      assert s1.charAt(count) != s2.charAt(i) : "Characters must not match.";
      ++i;
    }
  }
}
