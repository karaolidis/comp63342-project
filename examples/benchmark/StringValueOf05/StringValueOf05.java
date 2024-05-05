public class StringValueOf05 {
  public static void test(String str) {
    if (str.length() < 1) return;

    char characterValue = str.charAt(0);
    String tmp = String.valueOf(characterValue);
    assert tmp.equals("A");
  }
}
