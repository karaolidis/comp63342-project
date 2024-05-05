public class SubString03 {
  public static void test(String letters) {
    String tmp = letters.substring(9, 13);
    assert tmp.equals("teest");
  }
}
