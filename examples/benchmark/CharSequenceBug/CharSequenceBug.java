public class CharSequenceBug {
  public static void test(String s, CharSequence target) {
    String replaced = "";
    if (target.length() == 1)
      replaced = s.replace('b', 'c');
    assert replaced.indexOf('b') == -1 : "The string still contains 'b' after replacement";
  }
}
