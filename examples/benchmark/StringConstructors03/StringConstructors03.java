public class StringConstructors03 {
  public static void test(String arg0, String arg1) {
    String[] args = new String[2];
    args[0] = arg0;
    args[1] = arg1;

    String s = new String(args[0]);
    String s2 = new String(s);
    assert s2.equals(args[1]);
  }
}
