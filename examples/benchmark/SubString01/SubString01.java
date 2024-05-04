public class SubString01 {
  public static void test(String letters) {
    String tmp = letters.substring(20);
    assert tmp.equals("ganddroppingthem");
    tmp = letters.substring(6, 10);
    assert tmp.equals("file");
  }
}
