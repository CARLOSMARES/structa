import 'reflect-metadata';
import { StructaApp } from '@structa/runtime';
import { AppModule } from './modules/app.module';

/**
 * test-api Application Entry Point
 */
async function bootstrap() {
    const app = new StructaApp({
        port: parseInt(process.env.PORT || '3000'),
        host: process.env.HOST || '0.0.0.0',
    });
    
    await app.register(AppModule);
    await app.listen();
    
    console.log(`🚀 {name} is running on http://{app.host}:{app.port}`);
    console.log(`📚 API Documentation: http://{app.host}:{app.port}/docs`);
}

bootstrap().catch(console.error);
