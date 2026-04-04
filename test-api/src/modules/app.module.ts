import 'reflect-metadata';
import { Module } from '@structa/runtime';
import { UserController } from '../controllers/user.controller';
import { UserService } from '../services/user.service';

@Module({
    controllers: [UserController],
    providers: [UserService],
    exports: [UserService]
})
export class AppModule {}
