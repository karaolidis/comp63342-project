public class CharSequenceToString {
  public static void test(String arg) {
    CharSequence cs = arg;
    String s = cs.toString();
    int i = -1;
    if (s.equals("case1"))
      i = cs.length();
    assert i == -1 || i == 5;
  }
}
