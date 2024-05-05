class char1 {

  public static void test(String arg) {
    if (arg.length() < 1) return;
    char my_char = arg.charAt(0);
    int x = my_char;
    assert x >= 0 && x <= '\uffff';

    my_char = '\uffff';
    my_char++;
    assert my_char == 0;
  }
}
