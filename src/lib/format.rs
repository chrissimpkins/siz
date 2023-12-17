use humansize::{make_format, BINARY, DECIMAL};

#[inline(always)]
pub fn build_metric_size_formatter() -> impl Fn(u64) -> String {
    make_format(DECIMAL)
}

#[inline(always)]
pub fn build_binary_size_formatter() -> impl Fn(u64) -> String {
    make_format(BINARY)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_metric_size_fmt() {
        let msf = build_metric_size_formatter();
        assert_eq!(msf(1), "1 B");
        assert_eq!(msf(10), "10 B");
        assert_eq!(msf(100), "100 B");
        assert_eq!(msf(1000), "1 kB");
        assert_eq!(msf(10000), "10 kB");
        assert_eq!(msf(100000), "100 kB");
        assert_eq!(msf(1000000), "1 MB");
        assert_eq!(msf(10000000), "10 MB");
        assert_eq!(msf(100000000), "100 MB");
        assert_eq!(msf(1000000000), "1 GB");
    }

    #[test]
    fn test_binary_size_fmt() {
        let bsf = build_binary_size_formatter();
        assert_eq!(bsf(1), "1 B");
        assert_eq!(bsf(10), "10 B");
        assert_eq!(bsf(100), "100 B");
        assert_eq!(bsf(1000), "1000 B");
        assert_eq!(bsf(1024), "1 KiB");
        assert_eq!(bsf(10000), "9.77 KiB");
        assert_eq!(bsf(10240), "10 KiB");
        assert_eq!(bsf(100000), "97.66 KiB");
        assert_eq!(bsf(102400), "100 KiB");
        assert_eq!(bsf(1000000), "976.56 KiB");
        assert_eq!(bsf(1048576), "1 MiB");
        assert_eq!(bsf(10000000), "9.54 MiB");
        assert_eq!(bsf(10485760), "10 MiB");
        assert_eq!(bsf(100000000), "95.37 MiB");
        assert_eq!(bsf(104857600), "100 MiB");
        assert_eq!(bsf(1000000000), "953.67 MiB");
        assert_eq!(bsf(1073742000), "1.00 GiB");
    }
}