%define debug_package %{nil}

Name: kvp
Summary: Implementation of a https://kvp.network node in Rust based on the Substrate framework.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

Requires: systemd, shadow-utils
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}


%prep
%setup -q


%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%post
config_file="/etc/default/kvp"
getent group kvp >/dev/null || groupadd -r kvp
getent passwd kvp >/dev/null || \
    useradd -r -g kvp -d /home/kvp -m -s /sbin/nologin \
    -c "User account for running kvp as a service" kvp
if [ ! -e "$config_file" ]; then
    echo 'kvp_CLI_ARGS=""' > /etc/default/kvp
fi
exit 0

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/usr/lib/systemd/system/kvp.service
