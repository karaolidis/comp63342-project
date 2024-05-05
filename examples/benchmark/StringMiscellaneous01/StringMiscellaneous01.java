public class StringMiscellaneous01 {
  public static void test(String s1, String s2, String s3) {
    char[] charArray = new char[5];

    assert s1.length() == 25;

    int i = 0;
    for (int count = s1.length() - 1; count >= 0; count--) {
      System.out.printf("%c ", s1.charAt(count));
      assert s1.charAt(count) == s2.charAt(i);
      ++i;
    }

    // copy characters from string into charArray
    s1.getChars(0, 5, charArray, 0);
    i = 0;
    for (char character : charArray) {
      System.out.print(character);
      assert s3.charAt(i) == character;
      ++i;
    }

    System.out.println();
  }
}
