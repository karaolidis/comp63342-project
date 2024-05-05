public class StringConcatenation01 {
  public static void test(String arg1, String arg2) {
    String[] args = new String[2];
    args[0] = arg1;
    args[1] = arg2;

    String s1 = args[0];
    String s2 = args[1];

    assert s1.equals(args[0]);
    assert s2.equals(args[1]);

    String tmp = s1.concat(s2);
    assert tmp.equals(args[0] + args[1]);

    tmp = s1;
    assert tmp.equals(args[0]);
  }
}
