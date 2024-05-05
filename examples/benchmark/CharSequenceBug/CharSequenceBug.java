public class CharSequenceBug {
  public static void test(String s) {
    CharSequence target = "b";
    String replaced = "";
    if (target.length() == 1) replaced = s.replace('b', 'c');
    assert replaced.indexOf('b') != -1;
  }
}
