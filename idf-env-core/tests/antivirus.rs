#[cfg(test)]
mod tests {
    use idf_env_core::antivirus::get_antivirus_name;

    #[test]
    fn test_get_antivirus_property() {
        let result = get_antivirus_name();
    }
}
