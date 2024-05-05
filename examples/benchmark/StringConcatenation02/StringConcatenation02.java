public class StringConcatenation02 {

  public static void test(String s1, String s2) {
    String[] args = new String[2];
    args[0] = s1;
    args[1] = s2;

    assert s1.equals(args[0] + " ");
    assert s2.equals(args[1]);
  }
}
