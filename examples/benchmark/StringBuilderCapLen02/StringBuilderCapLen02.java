public class StringBuilderCapLen02 {
  public static void test(String testString) {
    StringBuilder buffer = new StringBuilder(testString);
    assert buffer.toString().equals("Diffblue is leader in automatic test case generation");
  }
}
