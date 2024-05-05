public class StringCompare01 {
    public static void test(String[] args) {
        String s1 = new String("test");
        String s2 = "goodbye";
        String s3 = "Automatic Test Generation";
        String s4 = "automatic test generation";

        assert s1.equals("test"); // true

        assert s1 != "test"; // true; they are not the same object

        assert s3.equalsIgnoreCase(s4); // true

        assert s1.compareTo(s2) == 13; // true

        assert s2.compareTo(s1) == -13; // true

        assert s1.compareTo(s1) == 0; // true

        assert s3.compareTo(s4) == -32; // true

        assert s4.compareTo(s3) == 32; // true

        // test regionMatches (case sensitive)
        assert !s3.regionMatches(0, s4, 0, 5); // true

        // test regionMatches (ignore case)
        assert s3.regionMatches(true, 0, s4, 0, 5); // true
    }
}
