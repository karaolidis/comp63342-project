public class StringConstructors03 {
  public static void test(String arg1, String arg2) {
    String s = new String(arg1);
    String s2 = new String(s);
    assert s2.equals(arg2);
  }
}
