package io.drogue.iot.hackathon.utils;

import java.nio.charset.Charset;
import java.util.List;
import java.util.Map;

import io.vertx.codegen.annotations.Nullable;
import io.vertx.core.AsyncResult;
import io.vertx.core.Handler;
import io.vertx.core.MultiMap;
import io.vertx.core.Vertx;
import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.Cookie;
import io.vertx.core.http.HttpMethod;
import io.vertx.core.http.HttpServerRequest;
import io.vertx.core.http.HttpServerResponse;
import io.vertx.ext.auth.User;
import io.vertx.ext.web.FileUpload;
import io.vertx.ext.web.ParsedHeaderValues;
import io.vertx.ext.web.RequestBody;
import io.vertx.ext.web.Route;
import io.vertx.ext.web.RoutingContext;

public class DummyRoutingContext implements RoutingContext {
    @Override
    public HttpServerRequest request() {
        return null;
    }

    @Override
    public HttpServerResponse response() {
        return null;
    }

    @Override
    public void next() {

    }

    @Override
    public void fail(int statusCode) {

    }

    @Override
    public void fail(Throwable throwable) {

    }

    @Override
    public void fail(int statusCode, Throwable throwable) {

    }

    @Override
    public RoutingContext put(String key, Object obj) {
        return null;
    }

    @Override
    public <T> @Nullable T get(String key) {
        return null;
    }

    @Override
    public <T> T get(String key, T defaultValue) {
        return null;
    }

    @Override
    public <T> @Nullable T remove(String key) {
        return null;
    }

    @Override
    public Map<String, Object> data() {
        return null;
    }

    @Override
    public Vertx vertx() {
        return null;
    }

    @Override
    public @Nullable String mountPoint() {
        return null;
    }

    @Override
    public @Nullable Route currentRoute() {
        return null;
    }

    @Override
    public String normalizedPath() {
        return null;
    }

    @Override
    public @Nullable Cookie getCookie(String name) {
        return null;
    }

    @Override
    public RoutingContext addCookie(Cookie cookie) {
        return null;
    }

    @Override
    public @Nullable Cookie removeCookie(String name, boolean invalidate) {
        return null;
    }

    @Override
    public int cookieCount() {
        return 0;
    }

    @Override
    public Map<String, Cookie> cookieMap() {
        return null;
    }

    @Override
    public RequestBody body() {
        return null;
    }

    @Override
    public List<FileUpload> fileUploads() {
        return null;
    }

    @Override
    public io.vertx.ext.web.@Nullable Session session() {
        return null;
    }

    @Override
    public boolean isSessionAccessed() {
        return false;
    }

    @Override
    public @Nullable User user() {
        return null;
    }

    @Override
    public @Nullable Throwable failure() {
        return null;
    }

    @Override
    public int statusCode() {
        return 0;
    }

    @Override
    public @Nullable String getAcceptableContentType() {
        return null;
    }

    @Override
    public ParsedHeaderValues parsedHeaders() {
        return null;
    }

    @Override
    public int addHeadersEndHandler(Handler<Void> handler) {
        return 0;
    }

    @Override
    public boolean removeHeadersEndHandler(int handlerID) {
        return false;
    }

    @Override
    public int addBodyEndHandler(Handler<Void> handler) {
        return 0;
    }

    @Override
    public boolean removeBodyEndHandler(int handlerID) {
        return false;
    }

    @Override
    public int addEndHandler(Handler<AsyncResult<Void>> handler) {
        return 0;
    }

    @Override
    public boolean removeEndHandler(int handlerID) {
        return false;
    }

    @Override
    public boolean failed() {
        return false;
    }

    @Override
    public void setBody(Buffer body) {

    }

    @Override
    public void setSession(io.vertx.ext.web.Session session) {

    }

    @Override
    public void setUser(User user) {

    }

    @Override
    public void clearUser() {

    }

    @Override
    public void setAcceptableContentType(@Nullable String contentType) {

    }

    @Override
    public void reroute(HttpMethod method, String path) {

    }

    @Override
    public Map<String, String> pathParams() {
        return null;
    }

    @Override
    public @Nullable String pathParam(String name) {
        return null;
    }

    @Override
    public MultiMap queryParams() {
        return null;
    }

    @Override
    public MultiMap queryParams(Charset encoding) {
        return null;
    }

    @Override
    public List<String> queryParam(String name) {
        return null;
    }
}
