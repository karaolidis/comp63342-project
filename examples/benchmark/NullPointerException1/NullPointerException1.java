class NullPointerException1 {
  public static void test(Object verifierObject) {
    Object o = null;
    o.hashCode();
    // should pass
    assert false;
  }
}
