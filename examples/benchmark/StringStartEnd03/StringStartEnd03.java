public class StringStartEnd03 {
  public static void test(String string1, String string2, String string3, String string4) {
    String[] strings = new String[4];
    strings[0] = string1;
    strings[1] = string2;
    strings[2] = string3;
    strings[3] = string4;

    int i = 0;
    for (String string : strings) {
      if (string.endsWith("ed"))
        ++i;
    }
    assert i == 3;
  }
}
