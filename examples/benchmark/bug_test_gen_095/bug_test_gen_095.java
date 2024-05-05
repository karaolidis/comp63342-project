public class bug_test_gen_095 {

  public static void test(String input) {
    StringBuilder sb = new StringBuilder(input);
    sb.append("Z");
    String s = sb.toString();
    assert (s.equals("fg"));
  }
}
