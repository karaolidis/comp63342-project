public class StringConstructors01 {
  public static void test(char[] charArray, String s) {
    String s1 = new String();
    String s2 = new String(s);
    String s3 = new String(charArray);
    String s4 = new String(charArray, 4, 4);

    assert s1.equals("");
    assert s2.equals("test");
    assert s3.equals("diffblue");
    assert s4.equals("blue");
  }
}
