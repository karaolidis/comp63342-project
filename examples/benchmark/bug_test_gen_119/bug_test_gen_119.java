public class bug_test_gen_119 {
  public static void test(boolean booleanValue) {
    String tmp = String.valueOf(booleanValue);
    if (booleanValue) assert tmp.equals("true");
    else assert tmp.equals("false");
  }
}
