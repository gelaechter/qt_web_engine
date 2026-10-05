extern "C" {

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    /// <p>Sets up an OpenGL Context that can be shared between threads. This has to be done before <a href="http://doc.qt.io/qt-5/qguiapplication.html">QGuiApplication</a> is created and before window's QPlatformOpenGLContext is created.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qtwebengine.html#initialize">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Sets up an OpenGL Context that can be shared between threads. This has to be done before <a href="http://doc.qt.io/qt-5/qguiapplication.html">QGuiApplication</a> is created and before window's QPlatformOpenGLContext is created.</p>
    /// <p>This has the same effect as setting the <a href="http://doc.qt.io/qt-5/qt.html#ApplicationAttribute-enum">Qt::AA_ShareOpenGLContexts</a> attribute with <a href="http://doc.qt.io/qt-5/qcoreapplication.html#setAttribute">QCoreApplication::setAttribute</a> before constructing <a href="http://doc.qt.io/qt-5/qguiapplication.html">QGuiApplication</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QtWebEngine_initialize();

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_metaObject(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_qt_metacast(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_qt_metacall(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    /// <p>Constructs a new profile with the parent <i>parent</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#QQuickWebEngineProfile">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs a new profile with the parent <i>parent</i>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_QQuickWebEngineProfile(
        parent: *mut ::qt_core::QObject,
    ) -> *mut crate::QQuickWebEngineProfile;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_dQQuickWebEngineProfile(
        this_ptr: *mut crate::QQuickWebEngineProfile,
    );

    /// <p>The storage name that is used to create separate subdirectories for each profile that uses the disk for storing persistent data and cache.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#storageName-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The storage name that is used to create separate subdirectories for each profile that uses the disk for storing persistent data and cache.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>storageName</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setStorageName</b></span>(const QString &amp;<i>name</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>storageNameChanged</b></span>()</td></tr>
    /// </tbody></table></div>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#persistentStoragePath-prop">persistentStoragePath</a> and <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#cachePath-prop">cachePath</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_storageName(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    /// <p>The storage name that is used to create separate subdirectories for each profile that uses the disk for storing persistent data and cache.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#storageName-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The storage name that is used to create separate subdirectories for each profile that uses the disk for storing persistent data and cache.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>storageName</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setStorageName</b></span>(const QString &amp;<i>name</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>storageNameChanged</b></span>()</td></tr>
    /// </tbody></table></div>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#persistentStoragePath-prop">persistentStoragePath</a> and <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#cachePath-prop">cachePath</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setStorageName(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        name: *const ::qt_core::QString,
    );

    /// <p>Whether the web engine profile is <i>off-the-record</i>. An off-the-record profile forces cookies, the HTTP cache, and other normally persistent data to be stored only in memory.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#offTheRecord-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Whether the web engine profile is <i>off-the-record</i>. An off-the-record profile forces cookies, the HTTP cache, and other normally persistent data to be stored only in memory.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> bool </td><td class="memItemRight bottomAlign"><span class="name"><b>isOffTheRecord</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setOffTheRecord</b></span>(bool <i>offTheRecord</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>offTheRecordChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_isOffTheRecord(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> bool;

    /// <p>Whether the web engine profile is <i>off-the-record</i>. An off-the-record profile forces cookies, the HTTP cache, and other normally persistent data to be stored only in memory.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#offTheRecord-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Whether the web engine profile is <i>off-the-record</i>. An off-the-record profile forces cookies, the HTTP cache, and other normally persistent data to be stored only in memory.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> bool </td><td class="memItemRight bottomAlign"><span class="name"><b>isOffTheRecord</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setOffTheRecord</b></span>(bool <i>offTheRecord</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>offTheRecordChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setOffTheRecord(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        offTheRecord: bool,
    );

    /// <p>The path to the location where the persistent data for the browser and web content are stored. Persistent data includes persistent cookies, HTML5 local storage, and visited links.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#persistentStoragePath-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The path to the location where the persistent data for the browser and web content are stored. Persistent data includes persistent cookies, HTML5 local storage, and visited links.</p>
    /// <p>By default, the storage is located below <a href="http://doc.qt.io/qt-5/qstandardpaths.html#writableLocation">QStandardPaths::writableLocation</a>(<a href="http://doc.qt.io/qt-5/qstandardpaths.html#StandardLocation-enum">QStandardPaths::DataLocation</a>) in a directory named using <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#storageName-prop">storageName</a>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>persistentStoragePath</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setPersistentStoragePath</b></span>(const QString &amp;<i>path</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>persistentStoragePathChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_persistentStoragePath(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    /// <p>The path to the location where the persistent data for the browser and web content are stored. Persistent data includes persistent cookies, HTML5 local storage, and visited links.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#persistentStoragePath-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The path to the location where the persistent data for the browser and web content are stored. Persistent data includes persistent cookies, HTML5 local storage, and visited links.</p>
    /// <p>By default, the storage is located below <a href="http://doc.qt.io/qt-5/qstandardpaths.html#writableLocation">QStandardPaths::writableLocation</a>(<a href="http://doc.qt.io/qt-5/qstandardpaths.html#StandardLocation-enum">QStandardPaths::DataLocation</a>) in a directory named using <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#storageName-prop">storageName</a>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>persistentStoragePath</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setPersistentStoragePath</b></span>(const QString &amp;<i>path</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>persistentStoragePathChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setPersistentStoragePath(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        path: *const ::qt_core::QString,
    );

    /// <p>The path to the location where the profile's caches are stored, in particular the HTTP cache.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#cachePath-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The path to the location where the profile's caches are stored, in particular the HTTP cache.</p>
    /// <p>By default, the caches are stored below <a href="http://doc.qt.io/qt-5/qstandardpaths.html#writableLocation">QStandardPaths::writableLocation</a>(<a href="http://doc.qt.io/qt-5/qstandardpaths.html#StandardLocation-enum">QStandardPaths::CacheLocation</a>) in a directory named using <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#storageName-prop">storageName</a>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>cachePath</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setCachePath</b></span>(const QString &amp;<i>path</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>cachePathChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_cachePath(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    /// <p>The path to the location where the profile's caches are stored, in particular the HTTP cache.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#cachePath-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The path to the location where the profile's caches are stored, in particular the HTTP cache.</p>
    /// <p>By default, the caches are stored below <a href="http://doc.qt.io/qt-5/qstandardpaths.html#writableLocation">QStandardPaths::writableLocation</a>(<a href="http://doc.qt.io/qt-5/qstandardpaths.html#StandardLocation-enum">QStandardPaths::CacheLocation</a>) in a directory named using <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#storageName-prop">storageName</a>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>cachePath</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setCachePath</b></span>(const QString &amp;<i>path</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>cachePathChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setCachePath(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        path: *const ::qt_core::QString,
    );

    /// <p>The user-agent string sent with HTTP to identify the browser.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpUserAgent-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The user-agent string sent with HTTP to identify the browser.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>httpUserAgent</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setHttpUserAgent</b></span>(const QString &amp;<i>userAgent</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>httpUserAgentChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_httpUserAgent(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    /// <p>The user-agent string sent with HTTP to identify the browser.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpUserAgent-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The user-agent string sent with HTTP to identify the browser.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>httpUserAgent</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setHttpUserAgent</b></span>(const QString &amp;<i>userAgent</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>httpUserAgentChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setHttpUserAgent(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        userAgent: *const ::qt_core::QString,
    );

    /// <p>This enumeration describes the type of the HTTP cache.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpCacheType-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This enumeration describes the type of the HTTP cache.</p>
    /// <p>If the profile is off-the-record, <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#HttpCacheType-enum">MemoryHttpCache</a> is returned.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QQuickWebEngineProfile::HttpCacheType </td><td class="memItemRight bottomAlign"><span class="name"><b>httpCacheType</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setHttpCacheType</b></span>(<i>QQuickWebEngineProfile::HttpCacheType</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>httpCacheTypeChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_httpCacheType(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> crate::q_quick_web_engine_profile::HttpCacheType;

    /// <p>This enumeration describes the type of the HTTP cache.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpCacheType-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This enumeration describes the type of the HTTP cache.</p>
    /// <p>If the profile is off-the-record, <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#HttpCacheType-enum">MemoryHttpCache</a> is returned.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QQuickWebEngineProfile::HttpCacheType </td><td class="memItemRight bottomAlign"><span class="name"><b>httpCacheType</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setHttpCacheType</b></span>(<i>QQuickWebEngineProfile::HttpCacheType</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>httpCacheTypeChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setHttpCacheType(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        arg1: crate::q_quick_web_engine_profile::HttpCacheType,
    );

    /// <p>This enumeration describes the policy of cookie persistency. If the profile is off-the-record, <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#PersistentCookiesPolicy-enum">NoPersistentCookies</a> is returned.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#persistentCookiesPolicy-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This enumeration describes the policy of cookie persistency. If the profile is off-the-record, <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#PersistentCookiesPolicy-enum">NoPersistentCookies</a> is returned.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QQuickWebEngineProfile::PersistentCookiesPolicy </td><td class="memItemRight bottomAlign"><span class="name"><b>persistentCookiesPolicy</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setPersistentCookiesPolicy</b></span>(<i>QQuickWebEngineProfile::PersistentCookiesPolicy</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>persistentCookiesPolicyChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_persistentCookiesPolicy(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> crate::q_quick_web_engine_profile::PersistentCookiesPolicy;

    /// <p>This enumeration describes the policy of cookie persistency. If the profile is off-the-record, <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#PersistentCookiesPolicy-enum">NoPersistentCookies</a> is returned.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#persistentCookiesPolicy-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This enumeration describes the policy of cookie persistency. If the profile is off-the-record, <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#PersistentCookiesPolicy-enum">NoPersistentCookies</a> is returned.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QQuickWebEngineProfile::PersistentCookiesPolicy </td><td class="memItemRight bottomAlign"><span class="name"><b>persistentCookiesPolicy</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setPersistentCookiesPolicy</b></span>(<i>QQuickWebEngineProfile::PersistentCookiesPolicy</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>persistentCookiesPolicyChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setPersistentCookiesPolicy(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        arg1: crate::q_quick_web_engine_profile::PersistentCookiesPolicy,
    );

    /// <p>The maximum size of the HTTP cache. If <code>0</code>, the size will be controlled automatically by <a href="http://doc.qt.io/qt-5/qtwebengine-module.html">QtWebEngine</a>. The default value is <code>0</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpCacheMaximumSize-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The maximum size of the HTTP cache. If <code>0</code>, the size will be controlled automatically by <a href="http://doc.qt.io/qt-5/qtwebengine-module.html">QtWebEngine</a>. The default value is <code>0</code>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> int </td><td class="memItemRight bottomAlign"><span class="name"><b>httpCacheMaximumSize</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setHttpCacheMaximumSize</b></span>(int <i>maxSize</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>httpCacheMaximumSizeChanged</b></span>()</td></tr>
    /// </tbody></table></div>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpCacheType-prop">httpCacheType</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_httpCacheMaximumSize(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> ::std::os::raw::c_int;

    /// <p>The maximum size of the HTTP cache. If <code>0</code>, the size will be controlled automatically by <a href="http://doc.qt.io/qt-5/qtwebengine-module.html">QtWebEngine</a>. The default value is <code>0</code>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpCacheMaximumSize-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The maximum size of the HTTP cache. If <code>0</code>, the size will be controlled automatically by <a href="http://doc.qt.io/qt-5/qtwebengine-module.html">QtWebEngine</a>. The default value is <code>0</code>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> int </td><td class="memItemRight bottomAlign"><span class="name"><b>httpCacheMaximumSize</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setHttpCacheMaximumSize</b></span>(int <i>maxSize</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>httpCacheMaximumSizeChanged</b></span>()</td></tr>
    /// </tbody></table></div>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpCacheType-prop">httpCacheType</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setHttpCacheMaximumSize(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        maxSize: ::std::os::raw::c_int,
    );

    /// <p>The value of the Accept-Language HTTP request-header field.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpAcceptLanguage-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The value of the Accept-Language HTTP request-header field.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>httpAcceptLanguage</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setHttpAcceptLanguage</b></span>(const QString &amp;<i>httpAcceptLanguage</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>httpAcceptLanguageChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_httpAcceptLanguage(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    /// <p>The value of the Accept-Language HTTP request-header field.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpAcceptLanguage-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The value of the Accept-Language HTTP request-header field.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>httpAcceptLanguage</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setHttpAcceptLanguage</b></span>(const QString &amp;<i>httpAcceptLanguage</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>httpAcceptLanguageChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setHttpAcceptLanguage(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        httpAcceptLanguage: *const ::qt_core::QString,
    );

    /// <p>Removes the custom URL scheme <i>scheme</i> from the profile.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#removeUrlScheme">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the custom URL scheme <i>scheme</i> from the profile.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#removeUrlSchemeHandler">removeUrlSchemeHandler</a>().</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_removeUrlScheme(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        scheme: *const ::qt_core::QByteArray,
    );

    /// <p>Removes all custom URL scheme handlers installed in the profile.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#removeAllUrlSchemeHandlers">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes all custom URL scheme handlers installed in the profile.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_removeAllUrlSchemeHandlers(
        this_ptr: *mut crate::QQuickWebEngineProfile,
    );

    /// <p>Removes the profile's cache entries.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#clearHttpCache">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Removes the profile's cache entries.</p>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p>This function was introduced in Qt 5.7.</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qml-qtwebengine-webengineprofile.html#clearHttpCache-method">WebEngineProfile::clearHttpCache</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_clearHttpCache(
        this_ptr: *mut crate::QQuickWebEngineProfile,
    );

    /// <p>This property holds the languages used by the spell checker.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#spellCheckLanguages-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the languages used by the spell checker.</p>
    /// <p>This property was introduced in QtWebEngine 1.4.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QStringList </td><td class="memItemRight bottomAlign"><span class="name"><b><a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#spellCheckLanguages">spellCheckLanguages</a></b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setSpellCheckLanguages</b></span>(const QStringList &amp;<i>languages</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>spellCheckLanguagesChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setSpellCheckLanguages(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        languages: *const ::qt_core::QStringList,
    );

    /// <p>This property holds the languages used by the spell checker.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#spellCheckLanguages-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the languages used by the spell checker.</p>
    /// <p>This property was introduced in QtWebEngine 1.4.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QStringList </td><td class="memItemRight bottomAlign"><span class="name"><b><a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#spellCheckLanguages">spellCheckLanguages</a></b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setSpellCheckLanguages</b></span>(const QStringList &amp;<i>languages</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>spellCheckLanguagesChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_spellCheckLanguages(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> *mut ::qt_core::QStringList;

    /// <p>This property holds whether the web engine spell checker is enabled.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#spellCheckEnabled-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds whether the web engine spell checker is enabled.</p>
    /// <p>This property was introduced in QtWebEngine 1.4.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> bool </td><td class="memItemRight bottomAlign"><span class="name"><b>isSpellCheckEnabled</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setSpellCheckEnabled</b></span>(bool <i>enabled</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>spellCheckEnabledChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setSpellCheckEnabled(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        enabled: bool,
    );

    /// <p>This property holds whether the web engine spell checker is enabled.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#spellCheckEnabled-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds whether the web engine spell checker is enabled.</p>
    /// <p>This property was introduced in QtWebEngine 1.4.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> bool </td><td class="memItemRight bottomAlign"><span class="name"><b>isSpellCheckEnabled</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setSpellCheckEnabled</b></span>(bool <i>enabled</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>spellCheckEnabledChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_isSpellCheckEnabled(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> bool;

    /// <p>This property holds whether this profile is used for downloading and caching during global certificate verification when using the online certificate status protocol (OCSP), certificate revokation lists (CRLs), and authority information access (AIA), for example.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#useForGlobalCertificateVerification-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds whether this profile is used for downloading and caching during global certificate verification when using the online certificate status protocol (OCSP), certificate revokation lists (CRLs), and authority information access (AIA), for example.</p>
    /// <p>As long as one profile has this option enabled, all other profiles will be able to use it for certificate verification. Only one profile at a time can have this option enabled. It is recommended that the profile has a disk HTTP cache to avoid needlessly re-downloading.</p>
    /// <p>By default, no profile has this property enabled.</p>
    /// <p>Currently, only affects Linux/NSS installations, where having a profile with this role enables OCSP.</p>
    /// <p>This property was introduced in Qt 5.13.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> bool </td><td class="memItemRight bottomAlign"><span class="name"><b>isUsedForGlobalCertificateVerification</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setUseForGlobalCertificateVerification</b></span>(bool <i>b</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>useForGlobalCertificateVerificationChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setUseForGlobalCertificateVerification(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        b: bool,
    );

    /// <p>This property holds whether this profile is used for downloading and caching during global certificate verification when using the online certificate status protocol (OCSP), certificate revokation lists (CRLs), and authority information access (AIA), for example.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#useForGlobalCertificateVerification-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds whether this profile is used for downloading and caching during global certificate verification when using the online certificate status protocol (OCSP), certificate revokation lists (CRLs), and authority information access (AIA), for example.</p>
    /// <p>As long as one profile has this option enabled, all other profiles will be able to use it for certificate verification. Only one profile at a time can have this option enabled. It is recommended that the profile has a disk HTTP cache to avoid needlessly re-downloading.</p>
    /// <p>By default, no profile has this property enabled.</p>
    /// <p>Currently, only affects Linux/NSS installations, where having a profile with this role enables OCSP.</p>
    /// <p>This property was introduced in Qt 5.13.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> bool </td><td class="memItemRight bottomAlign"><span class="name"><b>isUsedForGlobalCertificateVerification</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setUseForGlobalCertificateVerification</b></span>(bool <i>b</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>useForGlobalCertificateVerificationChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_isUsedForGlobalCertificateVerification(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> bool;

    /// <p>The path to the location where the downloaded files are stored.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#downloadPath-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The path to the location where the downloaded files are stored.</p>
    /// <p>Overrides the default path used for download location, setting it to <i>path</i>.</p>
    /// <p>If set to an empty string, the default path is restored.</p>
    /// <p><b>Note: </b>By default, the download path is <a href="http://doc.qt.io/qt-5/qstandardpaths.html#StandardLocation-enum">QStandardPaths::DownloadLocation</a>.</p>
    /// <p>This property was introduced in QtWebEngine 1.9.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>downloadPath</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setDownloadPath</b></span>(const QString &amp;<i>path</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>downloadPathChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_downloadPath(
        this_ptr: *const crate::QQuickWebEngineProfile,
    ) -> *mut ::qt_core::QString;

    /// <p>The path to the location where the downloaded files are stored.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#downloadPath-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The path to the location where the downloaded files are stored.</p>
    /// <p>Overrides the default path used for download location, setting it to <i>path</i>.</p>
    /// <p>If set to an empty string, the default path is restored.</p>
    /// <p><b>Note: </b>By default, the download path is <a href="http://doc.qt.io/qt-5/qstandardpaths.html#StandardLocation-enum">QStandardPaths::DownloadLocation</a>.</p>
    /// <p>This property was introduced in QtWebEngine 1.9.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>downloadPath</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setDownloadPath</b></span>(const QString &amp;<i>path</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>downloadPathChanged</b></span>()</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_setDownloadPath(
        this_ptr: *mut crate::QQuickWebEngineProfile,
        path: *const ::qt_core::QString,
    );

    /// <p>Returns the default profile.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#defaultProfile">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the default profile.</p>
    /// <p>The default profile uses the storage name "Default".</p>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#storageName-prop">storageName</a>().</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_defaultProfile(
    ) -> *mut crate::QQuickWebEngineProfile;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_metaObject(
        this_ptr: *const crate::QQuickWebEngineScript,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_qt_metacast(
        this_ptr: *mut crate::QQuickWebEngineScript,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_qt_metacall(
        this_ptr: *mut crate::QQuickWebEngineScript,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    /// <p>Constructs a new QQuickWebEngineScript with the parent <i>parent</i>.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#QQuickWebEngineScript">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Constructs a new QQuickWebEngineScript with the parent <i>parent</i>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_QQuickWebEngineScript(
        parent: *mut ::qt_core::QObject,
    ) -> *mut crate::QQuickWebEngineScript;

    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_dQQuickWebEngineScript(
        this_ptr: *mut crate::QQuickWebEngineScript,
    );

    /// <p>Returns the script object as string.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#toString">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Returns the script object as string.</p>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_toString(
        this_ptr: *const crate::QQuickWebEngineScript,
    ) -> *mut ::qt_core::QString;

    /// <p>This property holds the name of the script.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#name-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the name of the script.</p>
    /// <p>Can be useful to retrieve a particular script from <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#userScripts-prop">QQuickWebEngineProfile::userScripts</a>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>name</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setName</b></span>(const QString &amp;<i>name</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>nameChanged</b></span>(const QString &amp;<i>name</i>)</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_name(
        this_ptr: *const crate::QQuickWebEngineScript,
    ) -> *mut ::qt_core::QString;

    /// <p>This property holds the remote source location of the user script (if any).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceUrl-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the remote source location of the user script (if any).</p>
    /// <p>Unlike <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceCode-prop">sourceCode</a>, this property allows referring to user scripts that are not already loaded in memory, for instance, when stored on disk.</p>
    /// <p>Setting this property will change the <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceCode-prop">sourceCode</a> of the script.</p>
    /// <p><b>Note: </b>At present, only file-based sources are supported.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QUrl </td><td class="memItemRight bottomAlign"><span class="name"><b>sourceUrl</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setSourceUrl</b></span>(const QUrl &amp;<i>url</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>sourceUrlChanged</b></span>(const QUrl &amp;<i>url</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceCode-prop">QQuickWebEngineScript::sourceCode</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_sourceUrl(
        this_ptr: *const crate::QQuickWebEngineScript,
    ) -> *mut ::qt_core::QUrl;

    /// <p>This property holds the JavaScript source code of the user script.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceCode-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the JavaScript source code of the user script.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>sourceCode</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setSourceCode</b></span>(const QString &amp;<i>code</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>sourceCodeChanged</b></span>(const QString &amp;<i>code</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceUrl-prop">QQuickWebEngineScript::sourceUrl</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_sourceCode(
        this_ptr: *const crate::QQuickWebEngineScript,
    ) -> *mut ::qt_core::QString;

    /// <p>This property holds the point in the loading process at which the script will be executed.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#injectionPoint-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the point in the loading process at which the script will be executed.</p>
    /// <p>The default value is <code>Deferred</code>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QQuickWebEngineScript::InjectionPoint </td><td class="memItemRight bottomAlign"><span class="name"><b>injectionPoint</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setInjectionPoint</b></span>(QQuickWebEngineScript::InjectionPoint <i>injectionPoint</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>injectionPointChanged</b></span>(QQuickWebEngineScript::InjectionPoint <i>injectionPoint</i>)</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_injectionPoint(
        this_ptr: *const crate::QQuickWebEngineScript,
    ) -> crate::q_quick_web_engine_script::InjectionPoint;

    /// <p>This property holds the world ID defining which isolated world the script is executed in.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#worldId-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the world ID defining which isolated world the script is executed in.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QQuickWebEngineScript::ScriptWorldId </td><td class="memItemRight bottomAlign"><span class="name"><b>worldId</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setWorldId</b></span>(QQuickWebEngineScript::ScriptWorldId <i>scriptWorldId</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>worldIdChanged</b></span>(QQuickWebEngineScript::ScriptWorldId <i>scriptWorldId</i>)</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_worldId(
        this_ptr: *const crate::QQuickWebEngineScript,
    ) -> crate::q_quick_web_engine_script::ScriptWorldId;

    /// <p>This property holds whether the script is executed on every frame or only on the main frame.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#runOnSubframes-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds whether the script is executed on every frame or only on the main frame.</p>
    /// <p>Set this property to <code>true</code> if the script is executed on every frame in the page, or <code>false</code> if it is only run for the main frame. The default value is <code>false</code>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> bool </td><td class="memItemRight bottomAlign"><span class="name"><b>runOnSubframes</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setRunOnSubframes</b></span>(bool <i>on</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>runOnSubframesChanged</b></span>(bool <i>on</i>)</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_runOnSubframes(
        this_ptr: *const crate::QQuickWebEngineScript,
    ) -> bool;

    /// <p>This property holds the name of the script.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#name-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the name of the script.</p>
    /// <p>Can be useful to retrieve a particular script from <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#userScripts-prop">QQuickWebEngineProfile::userScripts</a>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>name</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setName</b></span>(const QString &amp;<i>name</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>nameChanged</b></span>(const QString &amp;<i>name</i>)</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_setName(
        this_ptr: *mut crate::QQuickWebEngineScript,
        name: *const ::qt_core::QString,
    );

    /// <p>This property holds the remote source location of the user script (if any).</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceUrl-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the remote source location of the user script (if any).</p>
    /// <p>Unlike <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceCode-prop">sourceCode</a>, this property allows referring to user scripts that are not already loaded in memory, for instance, when stored on disk.</p>
    /// <p>Setting this property will change the <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceCode-prop">sourceCode</a> of the script.</p>
    /// <p><b>Note: </b>At present, only file-based sources are supported.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QUrl </td><td class="memItemRight bottomAlign"><span class="name"><b>sourceUrl</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setSourceUrl</b></span>(const QUrl &amp;<i>url</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>sourceUrlChanged</b></span>(const QUrl &amp;<i>url</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceCode-prop">QQuickWebEngineScript::sourceCode</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_setSourceUrl(
        this_ptr: *mut crate::QQuickWebEngineScript,
        url: *const ::qt_core::QUrl,
    );

    /// <p>This property holds the JavaScript source code of the user script.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceCode-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the JavaScript source code of the user script.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QString </td><td class="memItemRight bottomAlign"><span class="name"><b>sourceCode</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setSourceCode</b></span>(const QString &amp;<i>code</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>sourceCodeChanged</b></span>(const QString &amp;<i>code</i>)</td></tr>
    /// </tbody></table></div>
    /// <p><b>See also </b><a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#sourceUrl-prop">QQuickWebEngineScript::sourceUrl</a>.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_setSourceCode(
        this_ptr: *mut crate::QQuickWebEngineScript,
        code: *const ::qt_core::QString,
    );

    /// <p>This property holds the point in the loading process at which the script will be executed.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#injectionPoint-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the point in the loading process at which the script will be executed.</p>
    /// <p>The default value is <code>Deferred</code>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QQuickWebEngineScript::InjectionPoint </td><td class="memItemRight bottomAlign"><span class="name"><b>injectionPoint</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setInjectionPoint</b></span>(QQuickWebEngineScript::InjectionPoint <i>injectionPoint</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>injectionPointChanged</b></span>(QQuickWebEngineScript::InjectionPoint <i>injectionPoint</i>)</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_setInjectionPoint(
        this_ptr: *mut crate::QQuickWebEngineScript,
        injectionPoint: crate::q_quick_web_engine_script::InjectionPoint,
    );

    /// <p>This property holds the world ID defining which isolated world the script is executed in.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#worldId-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds the world ID defining which isolated world the script is executed in.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> QQuickWebEngineScript::ScriptWorldId </td><td class="memItemRight bottomAlign"><span class="name"><b>worldId</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setWorldId</b></span>(QQuickWebEngineScript::ScriptWorldId <i>scriptWorldId</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>worldIdChanged</b></span>(QQuickWebEngineScript::ScriptWorldId <i>scriptWorldId</i>)</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_setWorldId(
        this_ptr: *mut crate::QQuickWebEngineScript,
        scriptWorldId: crate::q_quick_web_engine_script::ScriptWorldId,
    );

    /// <p>This property holds whether the script is executed on every frame or only on the main frame.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#runOnSubframes-prop">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>This property holds whether the script is executed on every frame or only on the main frame.</p>
    /// <p>Set this property to <code>true</code> if the script is executed on every frame in the page, or <code>false</code> if it is only run for the main frame. The default value is <code>false</code>.</p>
    /// <p><b>Access functions:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> bool </td><td class="memItemRight bottomAlign"><span class="name"><b>runOnSubframes</b></span>() const</td></tr>
    /// <tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>setRunOnSubframes</b></span>(bool <i>on</i>)</td><td class="memItemRight bottomAlign">[see note below]</td></tr>
    /// </tbody></table></div>
    /// <p><b>Note: </b>This function can be invoked via the meta-object system and from QML. See <a href="http://doc.qt.io/qt-5/qobject.html#Q_INVOKABLE">Q_INVOKABLE</a>.</p>
    /// <p><b>Notifier signal:</b></p>
    /// <div class="table"><table class="alignedsummary">
    /// <tbody><tr><td class="memItemLeft topAlign rightAlign"> void </td><td class="memItemRight bottomAlign"><span class="name"><b>runOnSubframesChanged</b></span>(bool <i>on</i>)</td></tr>
    /// </tbody></table></div></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_setRunOnSubframes(
        this_ptr: *mut crate::QQuickWebEngineScript,
        on: bool,
    );

    /// <p>A web engine profile contains settings, scripts, persistent cookie policy, and the list of visited links shared by all web engine pages that belong to the profile.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>A web engine profile contains settings, scripts, persistent cookie policy, and the list of visited links shared by all web engine pages that belong to the profile.</p>
    /// <p>Information about visited links is stored together with persistent cookies and other persistent data in a storage determined by the <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#storageName-prop">storageName</a> property. Persistent data is stored in a subdirectory determined by the <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#persistentStoragePath-prop">persistentStoragePath</a> property and the cache in a subdirectory determined by the <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#cachePath-prop">cachePath</a> property. The <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#httpCacheType-prop">httpCacheType</a> property describes the type of the cache: <i>in-memory</i> or <i>on-disk</i>. If only the <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#storageName-prop">storageName</a> property is set, the other values are generated automatically based on it. If you specify any of the values manually, you should do it before creating any pages that belong to the profile.</p>
    /// <p>Profiles can be used to isolate pages from each other. A typical use case is a dedicated <i>off-the-record profile</i> for a <i>private browsing</i> mode. An off-the-record profile forces cookies, the HTTP cache, and other normally persistent data to be stored only in memory. The <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#offTheRecord-prop">offTheRecord</a> property holds whether a profile is off-the-record.</p>
    /// <p>The default profile can be accessed by <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#defaultProfile">defaultProfile</a>(). It is a built-in profile that all web pages not specifically created with another profile belong to.</p>
    /// <p>A <a href="http://doc.qt.io/qt-5/qml-qtwebengine-webengineprofile.html">WebEngineProfile</a> instance can be created and accessed from C++ through the QQuickWebEngineProfile class, which exposes further functionality in C++. This allows Qt Quick applications to intercept URL requests (QQuickWebEngineProfile::setRequestInterceptor), or register custom URL schemes (<a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#installUrlSchemeHandler">QQuickWebEngineProfile::installUrlSchemeHandler</a>).</p>
    /// <p>Spellchecking HTML form fields can be enabled per profile by setting the <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#spellCheckEnabled-prop">spellCheckEnabled</a> property and the current languages used for spellchecking can be set by using the <a href="http://doc.qt.io/qt-5/qquickwebengineprofile.html#spellCheckLanguages">spellCheckLanguages</a> property.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineProfile_QQuickWebEngineProfile1(
    ) -> *mut crate::QQuickWebEngineProfile;

    /// <p>The QQuickWebEngineScript type enables the programmatic injection of so called <i>user scripts</i> in the JavaScript engine at different points, determined by <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#injectionPoint-prop">injectionPoint</a>, during the loading of web content.</p>
    ///
    /// <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html">C++ documentation</a>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>The QQuickWebEngineScript type enables the programmatic injection of so called <i>user scripts</i> in the JavaScript engine at different points, determined by <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#injectionPoint-prop">injectionPoint</a>, during the loading of web content.</p>
    /// <p>Scripts can be executed either in the main JavaScript <i>world</i>, along with the rest of the JavaScript coming from the web contents, or in their own isolated world. While the DOM of the page can be accessed from any world, JavaScript variables of a function defined in one world are not accessible from a different one. The <a href="http://doc.qt.io/qt-5/qquickwebenginescript.html#worldId-prop">worldId</a> property provides some predefined IDs for this purpose.</p></div>
    pub fn ctr_qt_web_engine_ffi_QQuickWebEngineScript_QQuickWebEngineScript1(
    ) -> *mut crate::QQuickWebEngineScript;

    pub fn ctr_qt_web_engine_ffi_static_cast_QQuickWebEngineProfile_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QQuickWebEngineProfile;

    pub fn ctr_qt_web_engine_ffi_static_cast_QObject_ptr(
        ptr: *mut crate::QQuickWebEngineProfile,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_ffi_dynamic_cast_QQuickWebEngineProfile_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QQuickWebEngineProfile;

    pub fn ctr_qt_web_engine_ffi_static_cast_QQuickWebEngineScript_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QQuickWebEngineScript;

    pub fn ctr_qt_web_engine_ffi_static_cast_QObject_ptr1(
        ptr: *mut crate::QQuickWebEngineScript,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_ffi_dynamic_cast_QQuickWebEngineScript_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::QQuickWebEngineScript;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_metaObject(
        this_ptr: *const crate::SlotOfScriptWorldId,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_qt_metacast(
        this_ptr: *mut crate::SlotOfScriptWorldId,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_qt_metacall(
        this_ptr: *mut crate::SlotOfScriptWorldId,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(*mut ::std::ffi::c_void, crate::q_quick_web_engine_script::ScriptWorldId),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfScriptWorldId;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_set(
        this_ptr: *mut crate::SlotOfScriptWorldId,
        callback: ::std::option::Option<
            extern "C" fn(*mut ::std::ffi::c_void, crate::q_quick_web_engine_script::ScriptWorldId),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_slot_(
        this_ptr: *mut crate::SlotOfScriptWorldId,
        arg0: crate::q_quick_web_engine_script::ScriptWorldId,
    );

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_metaObject(
        this_ptr: *const crate::SignalOfScriptWorldId,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_qt_metacast(
        this_ptr: *mut crate::SignalOfScriptWorldId,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_qt_metacall(
        this_ptr: *mut crate::SignalOfScriptWorldId,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId(
    ) -> *mut crate::SignalOfScriptWorldId;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_emit_(
        this_ptr: *mut crate::SignalOfScriptWorldId,
        arg0: crate::q_quick_web_engine_script::ScriptWorldId,
    );

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_metaObject(
        this_ptr: *const crate::SlotOfInjectionPoint,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_qt_metacast(
        this_ptr: *mut crate::SlotOfInjectionPoint,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_qt_metacall(
        this_ptr: *mut crate::SlotOfInjectionPoint,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint(
        parent: *mut ::qt_core::QObject,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                crate::q_quick_web_engine_script::InjectionPoint,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut crate::SlotOfInjectionPoint;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_set(
        this_ptr: *mut crate::SlotOfInjectionPoint,
        callback: ::std::option::Option<
            extern "C" fn(
                *mut ::std::ffi::c_void,
                crate::q_quick_web_engine_script::InjectionPoint,
            ),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_slot_(
        this_ptr: *mut crate::SlotOfInjectionPoint,
        arg0: crate::q_quick_web_engine_script::InjectionPoint,
    );

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_staticMetaObject(
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_metaObject(
        this_ptr: *const crate::SignalOfInjectionPoint,
    ) -> *const ::qt_core::QMetaObject;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_qt_metacast(
        this_ptr: *mut crate::SignalOfInjectionPoint,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_qt_metacall(
        this_ptr: *mut crate::SignalOfInjectionPoint,
        arg1: ::qt_core::q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::qt_core::QString;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint(
    ) -> *mut crate::SignalOfInjectionPoint;

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_emit_(
        this_ptr: *mut crate::SignalOfInjectionPoint,
        arg0: crate::q_quick_web_engine_script::InjectionPoint,
    );

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_dctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId(
        this_ptr: *mut crate::SlotOfScriptWorldId,
    );

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_dctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId(
        this_ptr: *mut crate::SignalOfScriptWorldId,
    );

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_dctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint(
        this_ptr: *mut crate::SlotOfInjectionPoint,
    );

    pub fn ctr_qt_web_engine_ffi_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_dctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint(
        this_ptr: *mut crate::SignalOfInjectionPoint,
    );

    pub fn ctr_qt_web_engine_ffi_static_cast_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfScriptWorldId;

    pub fn ctr_qt_web_engine_ffi_static_cast_QObject_ptr4(
        ptr: *mut crate::SlotOfScriptWorldId,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_ffi_dynamic_cast_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_ScriptWorldId_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfScriptWorldId;

    pub fn ctr_qt_web_engine_ffi_static_cast_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfScriptWorldId;

    pub fn ctr_qt_web_engine_ffi_static_cast_QObject_ptr5(
        ptr: *mut crate::SignalOfScriptWorldId,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_ffi_dynamic_cast_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_ScriptWorldId_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfScriptWorldId;

    pub fn ctr_qt_web_engine_ffi_static_cast_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfInjectionPoint;

    pub fn ctr_qt_web_engine_ffi_static_cast_QObject_ptr8(
        ptr: *mut crate::SlotOfInjectionPoint,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_ffi_dynamic_cast_ctr_qt_web_engine_ffi_slot_wrapper_QQuickWebEngineScript_InjectionPoint_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SlotOfInjectionPoint;

    pub fn ctr_qt_web_engine_ffi_static_cast_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfInjectionPoint;

    pub fn ctr_qt_web_engine_ffi_static_cast_QObject_ptr9(
        ptr: *mut crate::SignalOfInjectionPoint,
    ) -> *mut ::qt_core::QObject;

    pub fn ctr_qt_web_engine_ffi_dynamic_cast_ctr_qt_web_engine_ffi_signal_wrapper_QQuickWebEngineScript_InjectionPoint_ptr(
        ptr: *mut ::qt_core::QObject,
    ) -> *mut crate::SignalOfInjectionPoint;

}
