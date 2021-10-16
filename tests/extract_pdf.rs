use unembedx::extract_embedded_files;

#[test]
fn can_extract_embedded_pdf() {
    // This file is not an actual OOXML document, it contains only the
    // absolute bare minimum for this test to pass
    let test_pptx_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("testdata/dummy.pptx");

    let blank_pdf_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("testdata/blank.pdf");

    let target_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    #[cfg(feature = "filetypes")]
    let expected_file_path = target_dir.path().join("embedded_file_0.pdf");
    #[cfg(not(feature = "filetypes"))]
    let expected_file_path = target_dir.path().join("embedded_file_0");

    let file_count =
        extract_embedded_files(test_pptx_path, &target_dir).expect("Failed to extract file");

    assert_eq!(1, file_count);

    let expected_pdf = std::fs::read(blank_pdf_path).expect("Failed to read expected pdf");
    let actual_pdf = std::fs::read(expected_file_path).expect("Failed to read actual pdf");

    assert_eq!(actual_pdf, expected_pdf);
}
